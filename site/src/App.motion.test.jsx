import { render, screen, waitFor } from "@testing-library/react";
import { expect, test, vi } from "vitest";
import { I18nProvider } from "./i18n/context";

test("starts each tagline word at muted opacity when motion is allowed", async () => {
  window.matchMedia.mockImplementation((query) => ({
    matches: false,
    media: query,
    onchange: null,
    addEventListener: vi.fn(),
    removeEventListener: vi.fn(),
    addListener: vi.fn(),
    removeListener: vi.fn(),
    dispatchEvent: vi.fn(),
  }));
  const { default: App } = await import("./App");
  const { container } = render(
    <I18nProvider locale="ko">
      <App />
    </I18nProvider>,
  );
  const tagline = screen.getByRole("heading", {
    name: "터미널을 떠나지 않고, 분석하고 정리하고 다음 작업으로.",
  });
  const words = [...container.querySelectorAll(".tagline__word")];

  expect(tagline.textContent).toBe(
    "터미널을 떠나지 않고, 분석하고 정리하고 다음 작업으로.",
  );
  await waitFor(() => {
    const opacities = words.map((word) => Number(word.style.opacity));

    expect(opacities.every((opacity) => opacity >= 0.48)).toBe(true);
    expect(opacities.every((opacity) => opacity < 1)).toBe(true);
  });
});

test("skips entrance translation for the initial hash target", async () => {
  window.matchMedia.mockImplementation((query) => ({
    matches: false,
    media: query,
    onchange: null,
    addEventListener: vi.fn(),
    removeEventListener: vi.fn(),
    addListener: vi.fn(),
    removeListener: vi.fn(),
    dispatchEvent: vi.fn(),
  }));
  window.history.replaceState(null, "", "/cli-tools/#iterm-korean");

  try {
    const { default: App } = await import("./App");
    const { container } = render(
      <I18nProvider locale="ko">
        <App />
      </I18nProvider>,
    );
    const target = container.querySelector("#iterm-korean");

    expect(target.style.opacity).toBe("");
    expect(target.style.transform).toBe("");
    expect(target.style.filter).toBe("");
  } finally {
    window.history.replaceState(null, "", "/cli-tools/");
  }
});
