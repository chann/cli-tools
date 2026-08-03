export const SITE_BASE = "/cli-tools/";

export const LOCALES = {
  ko: {
    prefix: "",
    htmlLang: "ko",
    ogLocale: "ko_KR",
    label: "한국어",
    shortLabel: "KO",
  },
  en: {
    prefix: "en",
    htmlLang: "en",
    ogLocale: "en_US",
    label: "English",
    shortLabel: "EN",
  },
  ja: {
    prefix: "ja",
    htmlLang: "ja",
    ogLocale: "ja_JP",
    label: "日本語",
    shortLabel: "JA",
  },
  zh: {
    prefix: "zh",
    htmlLang: "zh-Hans",
    ogLocale: "zh_CN",
    label: "简体中文",
    shortLabel: "中文",
  },
};

export function localeFromPath(pathname = "/") {
  const relativePath = pathname.startsWith(SITE_BASE)
    ? pathname.slice(SITE_BASE.length)
    : pathname.replace(/^\//, "");
  const [prefix] = relativePath.split("/");
  const match = Object.entries(LOCALES).find(([, locale]) => locale.prefix === prefix);
  return match?.[0] || "ko";
}

export function documentFromPath(pathname = "/") {
  if (pathname.endsWith("/privacy.html")) return "privacy";
  if (pathname.endsWith("/terms.html")) return "terms";
  return "landing";
}

export function localizedPath(locale, documentName = "landing") {
  const target = LOCALES[locale] || LOCALES.ko;
  const root = `${SITE_BASE}${target.prefix ? `${target.prefix}/` : ""}`;
  return documentName === "landing" ? root : `${root}${documentName}.html`;
}

export function absoluteLocalizedUrl(locale, documentName = "landing") {
  return `https://chann.github.io${localizedPath(locale, documentName)}`;
}
