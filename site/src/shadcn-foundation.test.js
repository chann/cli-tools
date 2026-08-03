import { existsSync, readFileSync } from "node:fs";
import { describe, expect, test } from "vitest";

describe("shadcn Select foundation", () => {
  test("uses an installed Radix shadcn Select in JavaScript mode", () => {
    expect(existsSync("components.json")).toBe(true);
    expect(existsSync("src/components/ui/select.jsx")).toBe(true);
    expect(existsSync("src/lib/utils.js")).toBe(true);

    const config = JSON.parse(readFileSync("components.json", "utf8"));
    const source = readFileSync("src/components/ui/select.jsx", "utf8");
    expect(config.rsc).toBe(false);
    expect(config.tsx).toBe(false);
    expect(config.aliases.ui).toBe("@/components/ui");
    expect(source).toContain('data-slot="select-trigger"');
    expect(source).toContain('data-slot="select-content"');
    expect(source).toContain('data-slot="select-item"');
  });

  test("keeps the standard 36px trigger and 12px horizontal inset", () => {
    const source = readFileSync("src/components/ui/select.jsx", "utf8");
    expect(source).toContain("px-3");
    expect(source).toContain("data-[size=default]:h-9");
    expect(source).toContain("<ChevronDownIcon");
  });

  test("bridges shadcn colors to existing semantic tokens", () => {
    const css = readFileSync("src/styles.css", "utf8");
    expect(css).toContain('@import "tailwindcss"');
    expect(css).toContain("--color-background: var(--bg)");
    expect(css).toContain("--color-popover: var(--surface)");
    expect(css).toContain("--color-ring: var(--focus)");
  });
});
