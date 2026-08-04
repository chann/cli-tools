import { afterEach, describe, expect, test, vi } from "vitest";
import { initChrome } from "./chrome";

function renderChrome() {
  document.body.innerHTML = `
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
  test("opens and closes the mobile menu with localized labels", () => {
    renderChrome();
    const cleanup = initChrome({ document });
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

  test("closes the mobile menu with Escape and restores focus", () => {
    renderChrome();
    const cleanup = initChrome({ document });
    const button = document.getElementById("menu-toggle");

    button.click();
    document.dispatchEvent(new KeyboardEvent("keydown", { key: "Escape" }));

    expect(document.documentElement.classList.contains("menu-open")).toBe(false);
    expect(document.activeElement).toBe(button);
    cleanup();
  });

  test("replays the footer wordmark when it re-enters the viewport", () => {
    const wordmark = document.createElement("div");
    wordmark.dataset.footerWordmark = "";
    document.body.append(wordmark);

    let handleIntersection;
    const observe = vi.fn();
    const disconnect = vi.fn();
    const IntersectionObserver = vi.fn(function MockObserver(callback) {
      handleIntersection = callback;
      this.observe = observe;
      this.disconnect = disconnect;
    });

    const cleanup = initChrome({ document, IntersectionObserver });

    expect(IntersectionObserver).toHaveBeenCalledWith(expect.any(Function), {
      threshold: 0.2,
    });
    expect(observe).toHaveBeenCalledWith(wordmark);
    expect(wordmark.hasAttribute("data-visible")).toBe(false);

    handleIntersection([
      { intersectionRatio: 0.2, isIntersecting: true, target: wordmark },
    ]);

    expect(wordmark.hasAttribute("data-visible")).toBe(true);

    handleIntersection([
      { intersectionRatio: 0.1, isIntersecting: true, target: wordmark },
    ]);

    expect(wordmark.hasAttribute("data-visible")).toBe(false);

    handleIntersection([
      { intersectionRatio: 0.2, isIntersecting: true, target: wordmark },
    ]);

    expect(wordmark.hasAttribute("data-visible")).toBe(true);
    expect(disconnect).not.toHaveBeenCalled();
    cleanup();
    expect(disconnect).toHaveBeenCalledTimes(1);
  });

  test("shows the footer wordmark when IntersectionObserver is unavailable", () => {
    const wordmark = document.createElement("div");
    wordmark.dataset.footerWordmark = "";
    document.body.append(wordmark);

    const cleanup = initChrome({ document, IntersectionObserver: null });

    expect(wordmark.hasAttribute("data-visible")).toBe(true);
    cleanup();
  });
});
