import { readFileSync } from "node:fs";
import { resolve } from "node:path";

const root = resolve(import.meta.dirname, "..");
const expected = {
  ko: {
    directory: "",
    lang: "ko",
    headline: "반복 명령은 줄이고,",
    canonical: "https://chann.github.io/cli-tools/",
  },
  en: {
    directory: "en/",
    lang: "en",
    headline: "Cut the repetitive commands.",
    canonical: "https://chann.github.io/cli-tools/en/",
  },
  ja: {
    directory: "ja/",
    lang: "ja",
    headline: "繰り返しのコマンドを減らし、",
    canonical: "https://chann.github.io/cli-tools/ja/",
  },
  zh: {
    directory: "zh/",
    lang: "zh-Hans",
    headline: "减少重复命令，",
    canonical: "https://chann.github.io/cli-tools/zh/",
  },
};

function read(relativePath) {
  return readFileSync(resolve(root, "dist", relativePath), "utf8");
}

function requireText(html, text, relativePath) {
  if (!html.includes(text)) {
    throw new Error(`${relativePath} is missing ${JSON.stringify(text)}`);
  }
}

const applicationAssets = new Set();

for (const locale of Object.values(expected)) {
  const landingPath = `${locale.directory}index.html`;
  const landing = read(landingPath);
  requireText(landing, `<html lang="${locale.lang}"`, landingPath);
  requireText(landing, locale.headline, landingPath);
  requireText(landing, `rel="canonical" href="${locale.canonical}"`, landingPath);
  requireText(landing, "data-language-select", landingPath);
  requireText(landing, "data-theme-menu", landingPath);
  if (landing.includes("cli-tools-locale:")) {
    throw new Error(`${landingPath} still contains its source locale marker`);
  }
  const applicationAsset = landing.match(/<script type="module"[^>]+src="([^"]+\.js)"/i)?.[1];
  if (!applicationAsset) throw new Error(`${landingPath} has no application asset`);
  applicationAssets.add(applicationAsset);

  for (const documentName of ["privacy", "terms"]) {
    const legalPath = `${locale.directory}${documentName}.html`;
    const legal = read(legalPath);
    requireText(legal, `<html lang="${locale.lang}"`, legalPath);
    requireText(legal, "data-locale-link", legalPath);
  }
}

if (applicationAssets.size !== 1) {
  throw new Error(`locale pages do not share one application asset: ${[...applicationAssets]}`);
}

const notFound = read("404.html");
const sitemap = read("sitemap.xml");
for (const locale of Object.values(expected)) {
  requireText(notFound, `data-error-locale="${locale.directory.replace("/", "") || "ko"}"`, "404.html");
  requireText(sitemap, locale.canonical, "sitemap.xml");
}

console.log("Verified 4 landing pages, 8 legal pages, localized 404, sitemap, and one shared application asset.");
