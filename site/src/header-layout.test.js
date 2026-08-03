import { readFileSync } from "node:fs";
import { afterEach, describe, expect, test } from "vitest";

afterEach(() => {
  document.head.innerHTML = "";
  document.body.innerHTML = "";
});

describe("header preference layout", () => {
  test("keeps quiet edge spacing around the compact preference controls", () => {
    const style = document.createElement("style");
    const header = document.createElement("header");
    const nav = document.createElement("div");
    const brand = document.createElement("a");
    const host = document.createElement("div");
    style.textContent = readFileSync("src/styles.css", "utf8");
    header.className = "site-header";
    nav.className = "nav-shell";
    brand.className = "brand";
    host.className = "preference-cluster preference-host";
    nav.append(brand, host);
    header.append(nav);
    document.head.append(style);
    document.body.append(header);

    expect(getComputedStyle(nav).paddingLeft).toBe("10px");
    expect(getComputedStyle(nav).paddingRight).toBe("10px");
    expect(getComputedStyle(brand).paddingLeft).toBe("10px");
    expect(getComputedStyle(host).gap).toBe("6px");
  });

  test("gives every fallback the shared trigger geometry and arrow inset", () => {
    const style = document.createElement("style");
    const fallback = document.createElement("span");
    const select = document.createElement("select");
    const css = readFileSync("src/styles.css", "utf8");
    style.textContent = css;
    fallback.className = "preference-fallback preference-fallback--theme";
    fallback.append(select);
    document.head.append(style);
    document.body.append(fallback);

    const selectStyle = getComputedStyle(select);
    expect(selectStyle.height).toBe("36px");
    expect(selectStyle.paddingRight).toBe("32px");
    expect(selectStyle.borderRadius).toBe("10px");
    expect(css).toMatch(/\.preference-fallback::after\s*{[^}]*right:\s*12px;/s);
  });
});
