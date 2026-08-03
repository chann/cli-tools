import { render, screen, waitFor } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { afterEach, describe, expect, test, vi } from "vitest";
import { I18nProvider } from "@/i18n/context";
import { PreferenceControls } from "./preferences";

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

function renderPreferences(overrides = {}) {
  const desktop = document.createElement("div");
  const mobile = document.createElement("div");
  desktop.dataset.preferenceHost = "desktop";
  mobile.dataset.preferenceHost = "mobile";
  document.body.append(desktop, mobile);

  const props = {
    hosts: { desktop, mobile },
    documentRef: document,
    storage: localStorage,
    media: createMedia(false),
    navigate: vi.fn(),
    ...overrides,
  };

  const result = render(
    <I18nProvider locale="ko">
      <PreferenceControls {...props} />
    </I18nProvider>,
  );

  return { ...result, ...props };
}

afterEach(() => {
  document.querySelectorAll("[data-preference-host]").forEach((host) => host.remove());
});

describe("shared preference controls", () => {
  test("uses the same shadcn Select trigger for both preferences and layouts", () => {
    renderPreferences();

    expect(screen.getAllByRole("combobox", { name: "언어 선택" })).toHaveLength(2);
    expect(screen.getAllByRole("combobox", { name: "테마 선택" })).toHaveLength(2);

    const triggers = [...document.querySelectorAll('[data-slot="select-trigger"]')];
    expect(triggers).toHaveLength(4);
    const normalizedClasses = triggers.map((trigger) =>
      trigger.className.replace(/w-\[[^\]]+\]/g, "").replace(/\s+/g, " ").trim(),
    );
    expect(new Set(normalizedClasses).size).toBe(1);
  });

  test("synchronizes and persists a theme selection", async () => {
    const user = userEvent.setup();
    renderPreferences();

    await user.click(screen.getAllByRole("combobox", { name: "테마 선택" })[0]);
    await user.click(await screen.findByRole("option", { name: "다크" }));

    await waitFor(() => expect(screen.queryByRole("listbox")).toBeNull());

    expect(document.documentElement.dataset.themeMode).toBe("dark");
    expect(document.documentElement.dataset.theme).toBe("dark");
    expect(localStorage.getItem("cli-tools-theme")).toBe("dark");
    expect(
      screen.getAllByRole("combobox", { name: "테마 선택" })[1].textContent,
    ).toContain("다크");
  });

  test("stores a locale and navigates from the compact language control", async () => {
    const user = userEvent.setup();
    const navigate = vi.fn();
    renderPreferences({ navigate });

    await user.click(screen.getAllByRole("combobox", { name: "언어 선택" })[0]);
    await user.click(await screen.findByRole("option", { name: "JA" }));

    expect(localStorage.getItem("cli-tools-locale")).toBe("ja");
    expect(navigate).toHaveBeenCalledWith("/cli-tools/ja/");
  });

  test("keeps theme changes and navigation working when storage is blocked", async () => {
    const user = userEvent.setup();
    const navigate = vi.fn();
    const storage = {
      getItem: vi.fn(() => {
        throw new Error("blocked");
      }),
      setItem: vi.fn(() => {
        throw new Error("blocked");
      }),
    };
    renderPreferences({ navigate, storage });

    await user.click(screen.getAllByRole("combobox", { name: "테마 선택" })[0]);
    await user.click(await screen.findByRole("option", { name: "다크" }));
    expect(document.documentElement.dataset.theme).toBe("dark");

    await user.click(screen.getAllByRole("combobox", { name: "언어 선택" })[0]);
    await user.click(await screen.findByRole("option", { name: "EN" }));
    expect(navigate).toHaveBeenCalledWith("/cli-tools/en/");
  });

  test("follows system changes only while system mode is active", async () => {
    const user = userEvent.setup();
    const media = createMedia(false);
    renderPreferences({ media });

    media.setMatches(true);
    expect(document.documentElement.dataset.theme).toBe("dark");

    await user.click(screen.getAllByRole("combobox", { name: "테마 선택" })[0]);
    await user.click(await screen.findByRole("option", { name: "라이트" }));
    media.setMatches(true);
    expect(document.documentElement.dataset.theme).toBe("light");
  });
});
