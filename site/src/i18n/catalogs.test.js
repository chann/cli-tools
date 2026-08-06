import { describe, expect, test } from "vitest";
import { catalogs, getMessages } from "./catalogs";
import { LOCALES, localeFromPath, localizedPath } from "./locale";

function flattenShape(value, prefix = "") {
  if (Array.isArray(value)) {
    return value.flatMap((item, index) => flattenShape(item, `${prefix}[${index}]`));
  }
  if (value && typeof value === "object") {
    return Object.entries(value).flatMap(([key, item]) =>
      flattenShape(item, prefix ? `${prefix}.${key}` : key),
    );
  }
  return [prefix];
}

describe("locale contract", () => {
  test("maps localized URLs and equivalent documents", () => {
    expect(localeFromPath("/cli-tools/")).toBe("ko");
    expect(localeFromPath("/cli-tools/en/")).toBe("en");
    expect(localeFromPath("/cli-tools/ja/privacy.html")).toBe("ja");
    expect(localeFromPath("/cli-tools/zh/terms.html")).toBe("zh");
    expect(localizedPath("ko", "privacy")).toBe("/cli-tools/privacy.html");
    expect(localizedPath("zh", "landing")).toBe("/cli-tools/zh/");
    expect(localizedPath("en", "terms")).toBe("/cli-tools/en/terms.html");
  });

  test("defines Korean-first metadata for four supported locales", () => {
    expect(Object.keys(LOCALES)).toEqual(["ko", "en", "ja", "zh"]);
    expect(LOCALES.ko.prefix).toBe("");
    expect(LOCALES.zh.htmlLang).toBe("zh-Hans");
    expect(LOCALES.zh.label).toBe("简体中文");
  });
});

describe("message catalogs", () => {
  test("keep an identical complete key shape", () => {
    const koreanShape = flattenShape(catalogs.ko).sort();

    for (const locale of Object.keys(LOCALES)) {
      expect(flattenShape(catalogs[locale]).sort()).toEqual(koreanShape);
    }
  });

  test("provide representative copy in every language", () => {
    expect(getMessages("ko").hero.title[0]).toBe("반복 명령은 줄이고,");
    expect(getMessages("en").hero.title[0]).toBe("Cut the repetitive commands.");
    expect(getMessages("ja").hero.title[0]).toBe("繰り返しのコマンドを減らし、");
    expect(getMessages("zh").hero.title[0]).toBe("减少重复命令，");
    expect(getMessages("unsupported")).toBe(catalogs.ko);
  });

  test("keeps CLI commands and paths byte-identical", () => {
    const koreanCommands = catalogs.ko.tools.map((tool) => tool.examples);
    const koreanItermCommands = catalogs.ko.itermKeys.command;
    const koreanItermRestore = catalogs.ko.itermKeys.restoreCommand;
    const koreanGhosttyCommands = catalogs.ko.ghosttyKeys.command;
    const koreanGhosttyRestore = catalogs.ko.ghosttyKeys.restoreCommand;
    const koreanUtilities = catalogs.ko.utility.groups.flatMap((group) =>
      group.commands.map((command) => command.code),
    );

    for (const locale of Object.keys(LOCALES)) {
      expect(catalogs[locale].tools.map((tool) => tool.examples)).toEqual(koreanCommands);
      expect(catalogs[locale].itermKeys.command).toBe(koreanItermCommands);
      expect(catalogs[locale].itermKeys.restoreCommand).toBe(koreanItermRestore);
      expect(catalogs[locale].ghosttyKeys.command).toBe(koreanGhosttyCommands);
      expect(catalogs[locale].ghosttyKeys.restoreCommand).toBe(koreanGhosttyRestore);
      expect(
        catalogs[locale].utility.groups.flatMap((group) =>
          group.commands.map((command) => command.code),
        ),
      ).toEqual(koreanUtilities);
    }
  });
});
