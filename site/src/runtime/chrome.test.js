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
    <div data-theme-control>
      <button type="button" data-theme-option="system" aria-pressed="true">시스템</button>
      <button type="button" data-theme-option="light" aria-pressed="false">라이트</button>
      <button type="button" data-theme-option="dark" aria-pressed="false">다크</button>
    </div>
    <div data-theme-control>
      <button type="button" data-theme-option="system" aria-pressed="true">시스템</button>
      <button type="button" data-theme-option="light" aria-pressed="false">라이트</button>
      <button type="button" data-theme-option="dark" aria-pressed="false">다크</button>
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
        (button) => button.getAttribute("aria-pressed") === "true",
      ),
    ).toBe(true);
    expect(
      [...document.querySelectorAll('[data-theme-option="system"]')].every(
        (button) => button.getAttribute("aria-pressed") === "false",
      ),
    ).toBe(true);

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
