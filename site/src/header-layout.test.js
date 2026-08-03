import { readFileSync } from "node:fs";
import { afterEach, describe, expect, test } from "vitest";

afterEach(() => {
  document.head.innerHTML = "";
  document.body.innerHTML = "";
});

describe("header layout density", () => {
  test("adds quiet edge spacing and keeps the theme dropdown compact", () => {
    const style = document.createElement("style");
    const header = document.createElement("header");
    const nav = document.createElement("div");
    const brand = document.createElement("a");
    const trigger = document.createElement("button");
    const content = document.createElement("div");
    style.textContent = readFileSync("src/styles.css", "utf8");
    header.className = "site-header";
    nav.className = "nav-shell";
    brand.className = "brand";
    trigger.className = "theme-dropdown__trigger";
    content.className = "theme-dropdown__content";
    nav.append(brand, trigger, content);
    header.append(nav);
    document.head.append(style);
    document.body.append(header);

    const navStyle = getComputedStyle(nav);
    const triggerStyle = getComputedStyle(trigger);
    expect(navStyle.paddingLeft).toBe("10px");
    expect(navStyle.paddingRight).toBe("10px");
    expect(getComputedStyle(brand).paddingLeft).toBe("10px");
    expect(parseFloat(triggerStyle.minWidth)).toBeLessThanOrEqual(96);
    expect(triggerStyle.minHeight).toBe("36px");
    expect(getComputedStyle(content).width).toBe("136px");
  });
});
