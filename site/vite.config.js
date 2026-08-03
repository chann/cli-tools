import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";
import { fileURLToPath } from "node:url";
import {
  renderLanding,
  renderLegal,
  renderNotFound,
  renderSitemap,
} from "./src/i18n/render.js";

const root = fileURLToPath(new URL(".", import.meta.url));
const localeEntries = ["ko", "en", "ja", "zh"];

function localizedPages() {
  return {
    name: "cli-tools-localized-pages",
    transformIndexHtml: {
      order: "pre",
      handler(html) {
        const locale = html.match(/cli-tools-locale:(ko|en|ja|zh)/)?.[1];
        return locale ? renderLanding(locale) : html;
      },
    },
    generateBundle() {
      for (const locale of localeEntries) {
        const prefix = locale === "ko" ? "" : `${locale}/`;
        for (const documentName of ["privacy", "terms"]) {
          this.emitFile({
            type: "asset",
            fileName: `${prefix}${documentName}.html`,
            source: renderLegal(locale, documentName),
          });
        }
      }
      this.emitFile({
        type: "asset",
        fileName: "404.html",
        source: renderNotFound(),
      });
      this.emitFile({
        type: "asset",
        fileName: "sitemap.xml",
        source: renderSitemap(),
      });
    },
  };
}

export default defineConfig({
  base: "/cli-tools/",
  plugins: [localizedPages(), react()],
  build: {
    rollupOptions: {
      input: {
        main: `${root}index.html`,
        en: `${root}en/index.html`,
        ja: `${root}ja/index.html`,
        zh: `${root}zh/index.html`,
      },
    },
  },
  test: {
    environment: "jsdom",
    setupFiles: "./src/test/setup.js",
  },
});
