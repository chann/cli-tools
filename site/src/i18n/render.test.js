import { describe, expect, test } from "vitest";
import { readFileSync } from "node:fs";
import { JSDOM } from "jsdom";
import { getMessages } from "./catalogs";
import { LOCALES, absoluteLocalizedUrl, localizedPath } from "./locale";
import {
  escapeHtml,
  renderLanding,
  renderLegal,
  renderNotFound,
  renderSitemap,
} from "./render";

describe("localized static rendering", () => {
  test.each(Object.keys(LOCALES))("renders the %s landing shell and metadata", (locale) => {
    const messages = getMessages(locale);
    const html = renderLanding(locale);

    expect(html).toContain(`<html lang="${LOCALES[locale].htmlLang}" data-locale="${locale}">`);
    expect(html).toContain(`<title>${messages.meta.title}</title>`);
    expect(html).toContain(`href="${absoluteLocalizedUrl(locale)}"`);
    expect(html).toContain(`>${messages.hero.title[0]}<`);
    expect(html).toContain(`>${messages.hero.title[1]}<`);
    expect(html).toContain('rel="alternate" hreflang="ko"');
    expect(html).toContain('rel="alternate" hreflang="x-default"');
    expect(html).toContain('data-theme-menu');
    expect(html).toContain('role="menuitemradio"');
    expect(html).toContain('data-language-select');
    expect(html).toContain('type="application/ld+json"');
    expect(html).toContain(localizedPath(locale, "privacy"));
    expect(html).not.toMatch(/\{\{[^}]+\}\}/);
  });

  test.each(Object.keys(LOCALES))("renders localized %s legal documents", (locale) => {
    const messages = getMessages(locale);
    const privacy = renderLegal(locale, "privacy");
    const terms = renderLegal(locale, "terms");

    expect(privacy).toContain(`<html lang="${LOCALES[locale].htmlLang}"`);
    expect(privacy).toContain(`<h1>${messages.legal.privacy.title}</h1>`);
    expect(privacy).toContain(localizedPath(locale, "landing"));
    expect(privacy).toContain('data-locale-link');
    expect(privacy).toContain("cli-tools-theme");
    expect(terms).toContain(`<h1>${messages.legal.terms.title}</h1>`);
    expect(terms).toContain("MIT License");
  });

  test("renders a path-aware localized 404 and complete sitemap", () => {
    const notFound = renderNotFound();
    const sitemap = renderSitemap();

    expect(notFound).toContain("cli-tools-theme");
    expect(notFound).not.toContain("match(//");
    expect(notFound).toContain('location.pathname.split("/")[2]');

    for (const locale of Object.keys(LOCALES)) {
      expect(notFound).toContain(`data-error-locale="${locale}"`);
      expect(notFound).toContain(getMessages(locale).notFound.title);
      expect(notFound).toContain(getMessages(locale).notFound.navLabel);
      expect(notFound).toContain(getMessages(locale).shell.skip);
      expect(sitemap).toContain(absoluteLocalizedUrl(locale));
      expect(sitemap).toContain(absoluteLocalizedUrl(locale, "privacy"));
      expect(sitemap).toContain(absoluteLocalizedUrl(locale, "terms"));
    }
  });

  test("activates the matching locale when GitHub Pages serves the shared 404", () => {
    const dom = new JSDOM(renderNotFound(), {
      runScripts: "dangerously",
      url: "https://chann.github.io/cli-tools/ja/missing-page",
    });
    const { document } = dom.window;

    expect(document.documentElement.lang).toBe("ja");
    expect(document.documentElement.dataset.locale).toBe("ja");
    expect(document.title).toBe(getMessages("ja").notFound.description);
    expect(document.querySelector("[data-error-nav]").getAttribute("aria-label")).toBe(
      getMessages("ja").notFound.navLabel,
    );
    expect(document.querySelector("[data-error-skip]").textContent).toBe(
      getMessages("ja").shell.skip,
    );

    dom.window.close();
  });

  test("escapes catalog text before interpolation", () => {
    expect(escapeHtml(`<script>alert("x")</script> & 'quoted'`)).toBe(
      "&lt;script&gt;alert(&quot;x&quot;)&lt;/script&gt; &amp; &#39;quoted&#39;",
    );
  });

  test("keeps legal-page prose together", () => {
    const style = document.createElement("style");
    const code = document.createElement("code");
    const css = readFileSync("public/legal.css", "utf8");
    style.textContent = css;
    document.head.append(style);
    document.body.append(code);

    expect(getComputedStyle(document.body).wordBreak).toBe("keep-all");
    expect(getComputedStyle(document.body).overflowWrap).toBe("anywhere");
    expect(getComputedStyle(code).wordBreak).toBe("normal");
    expect(getComputedStyle(code).overflowWrap).toBe("normal");
    expect(css).toContain(':root[data-theme="light"]');
    expect(css).toContain(':root[data-theme="dark"]');

    code.remove();
    style.remove();
  });
});
