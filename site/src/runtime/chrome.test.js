import { afterEach, describe, expect, test, vi } from "vitest";
import { initChrome, resolveTheme } from "./chrome";

function createMedia(initialMatches = false) {
  const listeners = new Set();

  return {
    matches: initialMatches,
    addEventListener: vi.fn((type, listener) => {
      if (type === "change") listeners.add(listener);
    }),
    removeEventListener: vi.fn((type, listener) => {
      if (type === "change") listeners.delete(listener);
    }),
    setMatches(matches) {
      this.matches = matches;
      listeners.forEach((listener) => listener({ matches }));
    },
  };
}

function renderChrome() {
  document.body.innerHTML = `
    <div data-theme-menu>
      <button type="button" data-theme-trigger aria-haspopup="menu" aria-expanded="false">
        <span data-theme-current>시스템</span>
      </button>
      <div role="menu" hidden>
        <div role="group">
          <button type="button" role="menuitemradio" data-theme-option="system" aria-checked="true">시스템</button>
          <button type="button" role="menuitemradio" data-theme-option="light" aria-checked="false">라이트</button>
          <button type="button" role="menuitemradio" data-theme-option="dark" aria-checked="false">다크</button>
        </div>
      </div>
    </div>
    <select data-language-select aria-label="언어 선택">
      <option value="/cli-tools/" data-locale="ko" selected>KO</option>
      <option value="/cli-tools/en/" data-locale="en">EN</option>
      <option value="/cli-tools/ja/" data-locale="ja">JA</option>
      <option value="/cli-tools/zh/" data-locale="zh">中文</option>
    </select>
    <select data-language-select aria-label="언어 선택">
      <option value="/cli-tools/" data-locale="ko" selected>한국어</option>
      <option value="/cli-tools/en/" data-locale="en">English</option>
      <option value="/cli-tools/ja/" data-locale="ja">日本語</option>
      <option value="/cli-tools/zh/" data-locale="zh">简体中文</option>
    </select>
    <div data-theme-menu>
      <button type="button" data-theme-trigger aria-haspopup="menu" aria-expanded="false">
        <span data-theme-current>시스템</span>
      </button>
      <div role="menu" hidden>
        <div role="group">
          <button type="button" role="menuitemradio" data-theme-option="system" aria-checked="true">시스템</button>
          <button type="button" role="menuitemradio" data-theme-option="light" aria-checked="false">라이트</button>
          <button type="button" role="menuitemradio" data-theme-option="dark" aria-checked="false">다크</button>
        </div>
      </div>
    </div>
    <button
      id="menu-toggle"
      type="button"
      aria-expanded="false"
      aria-label="메뉴 열기"
      data-open-label="메뉴 열기"
      data-close-label="메뉴 닫기"
    >메뉴</button>
    <div id="mobile-menu" aria-hidden="true" inert>
      <a href="#tools">도구</a>
    </div>
  `;
}

afterEach(() => {
  document.body.innerHTML = "";
  document.documentElement.classList.remove("menu-open");
});

describe("site chrome", () => {
  test("resolves explicit and system theme modes", () => {
    expect(resolveTheme("system", false)).toBe("light");
    expect(resolveTheme("system", true)).toBe("dark");
    expect(resolveTheme("light", true)).toBe("light");
    expect(resolveTheme("dark", false)).toBe("dark");
  });

  test("applies a direct theme choice to every control and storage", () => {
    renderChrome();
    const media = createMedia(false);
    const cleanup = initChrome({ document, storage: localStorage, media });

    document.querySelector('[data-theme-option="dark"]').click();

    expect(document.documentElement.dataset.themeMode).toBe("dark");
    expect(document.documentElement.dataset.theme).toBe("dark");
    expect(localStorage.getItem("cli-tools-theme")).toBe("dark");
    expect(
      [...document.querySelectorAll('[data-theme-option="dark"]')].every(
        (button) => button.getAttribute("aria-checked") === "true",
      ),
    ).toBe(true);
    expect(
      [...document.querySelectorAll('[data-theme-option="system"]')].every(
        (button) => button.getAttribute("aria-checked") === "false",
      ),
    ).toBe(true);

    cleanup();
  });

  test("uses an accessible dropdown with radio choices and keyboard selection", () => {
    renderChrome();
    const cleanup = initChrome({
      document,
      storage: localStorage,
      media: createMedia(false),
    });
    const trigger = document.querySelector("[data-theme-trigger]");
    const menu = trigger.nextElementSibling;

    trigger.click();
    expect(trigger.getAttribute("aria-expanded")).toBe("true");
    expect(menu.hidden).toBe(false);
    expect(document.activeElement.dataset.themeOption).toBe("system");

    document.activeElement.dispatchEvent(
      new KeyboardEvent("keydown", { key: "End", bubbles: true }),
    );
    expect(document.activeElement.dataset.themeOption).toBe("dark");
    document.activeElement.click();

    expect(document.documentElement.dataset.themeMode).toBe("dark");
    expect(trigger.getAttribute("aria-expanded")).toBe("false");
    expect(menu.hidden).toBe(true);
    expect(trigger.querySelector("[data-theme-current]").textContent).toBe("다크");

    cleanup();
  });

  test("follows media changes only while system mode is selected", () => {
    renderChrome();
    const media = createMedia(false);
    const cleanup = initChrome({ document, storage: localStorage, media });

    media.setMatches(true);
    expect(document.documentElement.dataset.theme).toBe("dark");

    document.querySelector('[data-theme-option="light"]').click();
    media.setMatches(false);
    expect(document.documentElement.dataset.theme).toBe("light");
    media.setMatches(true);
    expect(document.documentElement.dataset.theme).toBe("light");

    cleanup();
  });

  test("applies theme changes when browser storage is unavailable", () => {
    renderChrome();
    const storage = {
      getItem: vi.fn(() => {
        throw new Error("blocked");
      }),
      setItem: vi.fn(() => {
        throw new Error("blocked");
      }),
    };
    const cleanup = initChrome({
      document,
      storage,
      media: createMedia(false),
    });

    document.querySelector('[data-theme-option="dark"]').click();

    expect(document.documentElement.dataset.themeMode).toBe("dark");
    expect(document.documentElement.dataset.theme).toBe("dark");

    cleanup();
  });

  test("stores a language choice, synchronizes selectors, and navigates", () => {
    renderChrome();
    const navigate = vi.fn();
    const cleanup = initChrome({
      document,
      storage: localStorage,
      media: createMedia(false),
      navigate,
    });
    const selectors = [...document.querySelectorAll("[data-language-select]")];

    selectors[0].value = "/cli-tools/ja/";
    selectors[0].dispatchEvent(new Event("change", { bubbles: true }));

    expect(localStorage.getItem("cli-tools-locale")).toBe("ja");
    expect(selectors.every((select) => select.value === "/cli-tools/ja/")).toBe(true);
    expect(navigate).toHaveBeenCalledWith("/cli-tools/ja/");

    cleanup();
  });

  test("navigates when language preference storage is unavailable", () => {
    renderChrome();
    const navigate = vi.fn();
    const storage = {
      getItem: vi.fn(() => null),
      setItem: vi.fn(() => {
        throw new Error("blocked");
      }),
    };
    const cleanup = initChrome({
      document,
      storage,
      media: createMedia(false),
      navigate,
    });
    const select = document.querySelector("[data-language-select]");

    select.value = "/cli-tools/en/";
    select.dispatchEvent(new Event("change", { bubbles: true }));

    expect(navigate).toHaveBeenCalledWith("/cli-tools/en/");
    cleanup();
  });

  test("opens and closes the mobile menu with localized labels", () => {
    renderChrome();
    const cleanup = initChrome({
      document,
      storage: localStorage,
      media: createMedia(false),
    });
    const button = document.getElementById("menu-toggle");
    const menu = document.getElementById("mobile-menu");

    button.click();
    expect(document.documentElement.classList.contains("menu-open")).toBe(true);
    expect(button.getAttribute("aria-expanded")).toBe("true");
    expect(button.getAttribute("aria-label")).toBe("메뉴 닫기");
    expect(menu.hasAttribute("inert")).toBe(false);

    menu.querySelector("a").click();
    expect(document.documentElement.classList.contains("menu-open")).toBe(false);
    expect(button.getAttribute("aria-expanded")).toBe("false");
    expect(button.getAttribute("aria-label")).toBe("메뉴 열기");
    expect(menu.hasAttribute("inert")).toBe(true);

    cleanup();
  });
});
