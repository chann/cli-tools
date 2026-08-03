import { getMessages } from "./catalogs.js";
import {
  LOCALES,
  absoluteLocalizedUrl,
  localizedPath,
} from "./locale.js";

export function escapeHtml(value) {
  return String(value)
    .replaceAll("&", "&amp;")
    .replaceAll("<", "&lt;")
    .replaceAll(">", "&gt;")
    .replaceAll('"', "&quot;")
    .replaceAll("'", "&#39;");
}

export function jsonForScript(value) {
  return JSON.stringify(value).replaceAll("<", "\\u003c");
}

function renderAlternates(documentName = "landing") {
  const links = Object.entries(LOCALES).map(
    ([locale, metadata]) =>
      `<link rel="alternate" hreflang="${metadata.htmlLang}" href="${absoluteLocalizedUrl(locale, documentName)}" />`,
  );
  links.push(
    `<link rel="alternate" hreflang="x-default" href="${absoluteLocalizedUrl("ko", documentName)}" />`,
  );
  return links.join("\n    ");
}

function renderLanguageSelect(locale, documentName, suffix, compact = false) {
  const messages = getMessages(locale);
  const options = Object.entries(LOCALES)
    .map(
      ([code, metadata]) =>
        `<option value="${localizedPath(code, documentName)}" data-locale="${code}"${code === locale ? " selected" : ""}>${escapeHtml(compact ? metadata.shortLabel : metadata.label)}</option>`,
    )
    .join("");

  return `<select class="language-select${compact ? " language-select--compact" : ""}" data-language-select aria-label="${escapeHtml(messages.shell.languageLabel)}" id="language-${suffix}">${options}</select>`;
}

function renderThemeDropdown(messages, id, modifier = "") {
  const choices = [
    ["system", messages.shell.themeSystem],
    ["light", messages.shell.themeLight],
    ["dark", messages.shell.themeDark],
  ];
  const items = choices
    .map(
      ([mode, label]) =>
        `<button type="button" role="menuitemradio" data-theme-option="${mode}" aria-checked="${mode === "system"}">${escapeHtml(label)}</button>`,
    )
    .join("");

  return `<div class="theme-dropdown${modifier}" data-theme-menu>
            <button class="theme-dropdown__trigger" type="button" data-theme-trigger aria-haspopup="menu" aria-controls="${id}" aria-expanded="false">
              <span class="theme-dropdown__icon" aria-hidden="true"></span>
              <span data-theme-current>${escapeHtml(messages.shell.themeSystem)}</span>
              <span class="theme-dropdown__chevron" aria-hidden="true"></span>
            </button>
            <div class="theme-dropdown__content" id="${id}" role="menu" aria-label="${escapeHtml(messages.shell.themeLabel)}" hidden>
              <div role="group" aria-label="${escapeHtml(messages.shell.themeLabel)}">${items}</div>
            </div>
          </div>`;
}

function renderThemeBootstrap() {
  return `<script>
      (() => {
        try {
          const allowed = ["system", "light", "dark"];
          const saved = localStorage.getItem("cli-tools-theme");
          const mode = allowed.includes(saved) ? saved : "system";
          const dark = mode === "dark" || (mode === "system" && matchMedia("(prefers-color-scheme: dark)").matches);
          document.documentElement.dataset.theme = dark ? "dark" : "light";
          document.documentElement.dataset.themeMode = mode;
        } catch {
          document.documentElement.dataset.theme = "light";
          document.documentElement.dataset.themeMode = "system";
        }
      })();
    </script>`;
}

function renderFaqData(messages) {
  return {
    "@context": "https://schema.org",
    "@type": "FAQPage",
    mainEntity: messages.faq.items.map((item) => ({
      "@type": "Question",
      name: item.question,
      acceptedAnswer: { "@type": "Answer", text: item.answer },
    })),
  };
}

export function renderLanding(locale = "ko") {
  const activeLocale = LOCALES[locale] ? locale : "ko";
  const metadata = LOCALES[activeLocale];
  const messages = getMessages(activeLocale);
  const canonical = absoluteLocalizedUrl(activeLocale);

  return `<!doctype html>
<html lang="${metadata.htmlLang}" data-locale="${activeLocale}">
  <head>
    <meta charset="UTF-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <meta name="description" content="${escapeHtml(messages.meta.description)}" />
    <meta name="theme-color" content="#f4f2ea" media="(prefers-color-scheme: light)" />
    <meta name="theme-color" content="#131209" media="(prefers-color-scheme: dark)" />
    <meta property="og:type" content="website" />
    <meta property="og:locale" content="${metadata.ogLocale}" />
    <meta property="og:url" content="${canonical}" />
    <meta property="og:title" content="${escapeHtml(messages.meta.title)}" />
    <meta property="og:description" content="${escapeHtml(messages.meta.socialDescription)}" />
    <meta property="og:image" content="https://chann.github.io/cli-tools/images/og-cli-tools-landing.png" />
    <meta property="og:image:width" content="1200" />
    <meta property="og:image:height" content="630" />
    <meta property="og:image:alt" content="${escapeHtml(messages.meta.imageAlt)}" />
    <meta name="twitter:card" content="summary_large_image" />
    <meta name="twitter:title" content="${escapeHtml(messages.meta.title)}" />
    <meta name="twitter:description" content="${escapeHtml(messages.meta.socialDescription)}" />
    <meta name="twitter:image" content="https://chann.github.io/cli-tools/images/og-cli-tools-landing.png" />
    <link rel="canonical" href="${canonical}" />
    ${renderAlternates("landing")}
    <link rel="icon" href="/cli-tools/favicon.svg" type="image/svg+xml" />
    <script type="application/ld+json">${jsonForScript(renderFaqData(messages))}</script>
    ${renderThemeBootstrap()}
    <title>${escapeHtml(messages.meta.title)}</title>
  </head>
  <body>
    <a class="skip-link" href="#main">${escapeHtml(messages.shell.skip)}</a>
    <header class="site-header">
      <div class="nav-shell">
        <a class="brand" href="#top" aria-label="${escapeHtml(messages.shell.brandHome)}" aria-current="page"><span>cli-tools</span></a>
        <nav class="nav-links" aria-label="${escapeHtml(messages.shell.navLabel)}">
          <a href="#tools">${escapeHtml(messages.shell.navTools)}</a>
          <a href="#install">${escapeHtml(messages.shell.navInstall)}</a>
          <a href="https://github.com/chann/cli-tools">GitHub</a>
        </nav>
        <div class="preference-cluster">
          ${renderLanguageSelect(activeLocale, "landing", "desktop", true)}
          ${renderThemeDropdown(messages, "theme-menu-desktop")}
        </div>
        <button class="menu-button" id="menu-toggle" type="button" aria-label="${escapeHtml(messages.shell.menuOpen)}" data-open-label="${escapeHtml(messages.shell.menuOpen)}" data-close-label="${escapeHtml(messages.shell.menuClose)}" aria-controls="mobile-menu" aria-expanded="false">
          <span aria-hidden="true"></span><span aria-hidden="true"></span>
        </button>
      </div>
    </header>
    <div class="mobile-menu" id="mobile-menu" aria-hidden="true" inert>
      <nav class="mobile-menu__content" aria-label="${escapeHtml(messages.shell.mobileNavLabel)}">
        <div class="mobile-menu__links">
          <a href="#tools">${escapeHtml(messages.shell.exploreTools)}</a>
          <a href="#install">${escapeHtml(messages.hero.action)}</a>
          <a href="https://github.com/chann/cli-tools">${escapeHtml(messages.shell.viewGitHub)}</a>
        </div>
        <div class="mobile-preferences">
          <p>${escapeHtml(messages.shell.languageLabel)}</p>
          ${renderLanguageSelect(activeLocale, "landing", "mobile")}
          <p>${escapeHtml(messages.shell.themeTitle)}</p>
          ${renderThemeDropdown(messages, "theme-menu-mobile", " theme-dropdown--mobile")}
        </div>
      </nav>
    </div>
    <main id="main">
      <section class="hero" id="top">
        <div class="hero__inner">
          <div class="hero__copy">
            <p class="hero__eyebrow">${escapeHtml(messages.hero.eyebrow)}</p>
            <h1>${escapeHtml(messages.hero.title[0])}<br /><span>${escapeHtml(messages.hero.title[1])}</span></h1>
            <p class="hero__summary">${escapeHtml(messages.hero.summary)}</p>
            <div class="hero__actions"><a class="button button--primary" href="#install">${escapeHtml(messages.hero.action)}</a></div>
            <div class="hero__meta" role="group" aria-label="${escapeHtml(messages.shell.projectInfo)}">${messages.hero.facts.map((fact) => `<span>${escapeHtml(fact)}</span>`).join("")}</div>
          </div>
          <div class="hero-terminal" role="img" aria-label="${escapeHtml(messages.hero.terminalLabel)}">
            <div class="hero-terminal__bar"><span>~/workspace/product</span><span>zsh</span></div>
            <div class="hero-terminal__body">
              <div class="terminal-command"><span class="terminal-prompt">$</span><code>zzz cargo test</code></div>
              <p class="terminal-result">started · pid 82417 · prompt returned</p>
              <div class="terminal-command"><span class="terminal-prompt">$</span><code>dev-tools json '{"b":2,"a":1}' --sort asc</code></div>
              <p class="terminal-output">{"a":1,"b":2}</p>
              <div class="terminal-command terminal-command--last"><span class="terminal-prompt">$</span><code>work-summary --month</code></div>
              <div class="terminal-stats" aria-hidden="true"><span>commits grouped</span><span>active days mapped</span><span>estimate ready</span></div>
            </div>
          </div>
        </div>
      </section>
      <div id="root"></div>
    </main>
    <footer class="site-footer">
      <div class="section-shell site-footer__inner">
        <a class="brand" href="#top"><span class="brand-mark" aria-hidden="true">&gt;_</span><span>cli-tools</span></a>
        <p>${escapeHtml(messages.shell.footerCopy)}</p>
        <nav class="footer-links" aria-label="${escapeHtml(messages.shell.footerNav)}">
          <a href="${localizedPath(activeLocale, "privacy")}">${escapeHtml(messages.shell.privacy)}</a>
          <a href="${localizedPath(activeLocale, "terms")}">${escapeHtml(messages.shell.terms)}</a>
          <a href="https://github.com/chann/cli-tools/blob/main/LICENSE">License</a>
          <a href="https://github.com/chann/cli-tools">GitHub</a>
        </nav>
      </div>
    </footer>
    <noscript>${escapeHtml(messages.shell.noScript)}</noscript>
    <script type="module" src="/src/main.jsx"></script>
  </body>
</html>`;
}

function renderLocaleLinks(locale, documentName) {
  return Object.entries(LOCALES)
    .map(
      ([code, metadata]) =>
        `<a href="${localizedPath(code, documentName)}" data-locale-link="${code}"${code === locale ? ' aria-current="page"' : ""}>${escapeHtml(metadata.label)}</a>`,
    )
    .join("");
}

function renderLocaleStorageScript() {
  return `<script>
      document.querySelectorAll("[data-locale-link]").forEach((link) => {
        link.addEventListener("click", () => {
          try { localStorage.setItem("cli-tools-locale", link.dataset.localeLink); } catch {}
        });
      });
    </script>`;
}

export function renderLegal(locale = "ko", kind = "privacy") {
  const activeLocale = LOCALES[locale] ? locale : "ko";
  const documentName = kind === "terms" ? "terms" : "privacy";
  const metadata = LOCALES[activeLocale];
  const messages = getMessages(activeLocale);
  const page = messages.legal[documentName];

  return `<!doctype html>
<html lang="${metadata.htmlLang}" data-locale="${activeLocale}">
  <head>
    <meta charset="UTF-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <meta name="robots" content="noindex,follow" />
    <meta name="description" content="${escapeHtml(page.description)}" />
    <link rel="canonical" href="${absoluteLocalizedUrl(activeLocale, documentName)}" />
    ${renderAlternates(documentName)}
    <link rel="icon" href="/cli-tools/favicon.svg" type="image/svg+xml" />
    <link rel="stylesheet" href="/cli-tools/legal.css" />
    ${renderThemeBootstrap()}
    <title>${escapeHtml(page.title)} | cli-tools</title>
  </head>
  <body>
    <a class="skip-link" href="#main">${escapeHtml(messages.shell.skip)}</a>
    <nav class="legal-nav" aria-label="${escapeHtml(messages.legal.navLabel)}">
      <a class="brand" href="${localizedPath(activeLocale)}"><span>cli-tools</span></a>
      <div class="legal-languages">${renderLocaleLinks(activeLocale, documentName)}</div>
      <a class="back-link" href="${localizedPath(activeLocale)}">${escapeHtml(messages.legal.backHome)}</a>
    </nav>
    <main class="legal-main" id="main">
      <article>
        <header class="legal-header">
          <p class="legal-label">${escapeHtml(messages.legal.updated)}</p>
          <h1>${escapeHtml(page.title)}</h1>
          <p class="legal-intro">${escapeHtml(page.intro)}</p>
        </header>
        <div class="legal-content">
          ${page.sections.map((section) => `<section><h2>${escapeHtml(section.title)}</h2><p>${escapeHtml(section.body)}</p>${documentName === "terms" && section === page.sections[0] ? '<p><a href="https://github.com/chann/cli-tools/blob/main/LICENSE">MIT License</a></p>' : ""}</section>`).join("")}
        </div>
      </article>
    </main>
    ${renderLocaleStorageScript()}
  </body>
</html>`;
}

export function renderNotFound() {
  const localeSections = Object.entries(LOCALES)
    .map(([locale]) => {
      const messages = getMessages(locale);
      return `<section data-error-locale="${locale}" data-title="${escapeHtml(messages.notFound.description)}" data-nav-label="${escapeHtml(messages.notFound.navLabel)}" data-skip-label="${escapeHtml(messages.shell.skip)}">
        <p class="error-code">404</p>
        <h1>${escapeHtml(messages.notFound.title)}</h1>
        <p class="legal-intro">${escapeHtml(messages.notFound.intro)}</p>
        <a class="primary-link" href="${localizedPath(locale)}">${escapeHtml(messages.notFound.action)}</a>
      </section>`;
    })
    .join("");

  return `<!doctype html>
<html lang="ko" data-locale="ko">
  <head>
    <meta charset="UTF-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <meta name="robots" content="noindex,nofollow" />
    <meta name="description" content="${escapeHtml(getMessages("ko").notFound.description)}" />
    <link rel="icon" href="/cli-tools/favicon.svg" type="image/svg+xml" />
    <link rel="stylesheet" href="/cli-tools/legal.css" />
    <style>
      [data-error-locale] { display: none; }
      html[data-locale="ko"] [data-error-locale="ko"], html[data-locale="en"] [data-error-locale="en"], html[data-locale="ja"] [data-error-locale="ja"], html[data-locale="zh"] [data-error-locale="zh"] { display: block; }
    </style>
    ${renderThemeBootstrap()}
    <script>
      (() => {
        const candidate = location.pathname.split("/")[2];
        const locale = ["en", "ja", "zh"].includes(candidate) ? candidate : "ko";
        const langs = { ko: "ko", en: "en", ja: "ja", zh: "zh-Hans" };
        document.documentElement.dataset.locale = locale;
        document.documentElement.lang = langs[locale];
      })();
    </script>
    <title>${escapeHtml(getMessages("ko").notFound.description)}</title>
  </head>
  <body>
    <a class="skip-link" data-error-skip href="#main">${escapeHtml(getMessages("ko").shell.skip)}</a>
    <nav class="legal-nav" data-error-nav aria-label="${escapeHtml(getMessages("ko").notFound.navLabel)}"><a class="brand" href="/cli-tools/"><span>cli-tools</span></a></nav>
    <main class="legal-main error-main" id="main">${localeSections}</main>
    <script>
      const activeError = document.querySelector('[data-error-locale="' + document.documentElement.dataset.locale + '"]');
      if (activeError) {
        document.title = activeError.dataset.title;
        document.querySelector('meta[name="description"]').content = activeError.dataset.title;
        document.querySelector('[data-error-nav]').setAttribute('aria-label', activeError.dataset.navLabel);
        document.querySelector('[data-error-skip]').textContent = activeError.dataset.skipLabel;
      }
    </script>
  </body>
</html>`;
}

export function renderSitemap() {
  const urls = Object.keys(LOCALES).flatMap((locale) =>
    ["landing", "privacy", "terms"].map(
      (documentName) => `  <url>
    <loc>${absoluteLocalizedUrl(locale, documentName)}</loc>
    <lastmod>2026-08-03</lastmod>
  </url>`,
    ),
  );
  return `<?xml version="1.0" encoding="UTF-8"?>
<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">
${urls.join("\n")}
</urlset>`;
}
