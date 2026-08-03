import { describe, expect, test, vi } from "vitest";
import {
  applyTheme,
  readPreference,
  resolveTheme,
  writePreference,
} from "./preferences";

describe("preference state", () => {
  test("resolves explicit and system themes", () => {
    expect(resolveTheme("system", false)).toBe("light");
    expect(resolveTheme("system", true)).toBe("dark");
    expect(resolveTheme("light", true)).toBe("light");
    expect(resolveTheme("dark", false)).toBe("dark");
  });

  test("applies both theme data attributes", () => {
    applyTheme(document, "dark", false);
    expect(document.documentElement.dataset.themeMode).toBe("dark");
    expect(document.documentElement.dataset.theme).toBe("dark");

    applyTheme(document, "unsupported", true);
    expect(document.documentElement.dataset.themeMode).toBe("system");
    expect(document.documentElement.dataset.theme).toBe("dark");
  });

  test("fails open when storage is blocked", () => {
    const storage = {
      getItem: vi.fn(() => {
        throw new Error("blocked");
      }),
      setItem: vi.fn(() => {
        throw new Error("blocked");
      }),
    };

    expect(readPreference(storage, "cli-tools-theme")).toBeNull();
    expect(() => writePreference(storage, "cli-tools-theme", "dark")).not.toThrow();
  });
});
