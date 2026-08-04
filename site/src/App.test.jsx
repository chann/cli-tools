import { fireEvent, render, screen, waitFor } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { existsSync, readFileSync } from "node:fs";
import { describe, expect, test, vi } from "vitest";
import App from "./App";
import { I18nProvider } from "./i18n/context";
import { renderLanding, renderLegal, renderNotFound } from "./i18n/render";

function renderApp(locale = "ko") {
  return render(
    <I18nProvider locale={locale}>
      <App />
    </I18nProvider>,
  );
}

describe("cli-tools website", () => {
  test("renders the core message and responsive hero without JavaScript", () => {
    const html = renderLanding("ko");
    const page = new DOMParser().parseFromString(html, "text/html");

    expect(html).toContain("<h1>반복 명령은 줄이고,");
    expect(html).toContain('id="menu-toggle"');
    expect(html).toContain('class="hero-terminal"');
    expect(html).toContain("설치 명령 보기");
    expect(html).toContain("로그인 없음");
    expect(html).toContain("og-cli-tools-landing.png");
    expect(html).toContain('type="application/ld+json"');
    expect(html).toContain("twitter:card");
    expect(html).toContain("https://chann.github.io/cli-tools/");
    expect(html).not.toContain("channprj.github.io");
    expect(html).not.toMatch(/[—–]/);

    const header = page.querySelector(".site-header");
    expect(header.textContent).toContain("cli-tools");
    expect(header.querySelector(".brand-mark")).toBeNull();
    expect(header.querySelector(".preference-cluster")).not.toBeNull();
    expect(page.querySelectorAll("[data-preference-host]")).toHaveLength(2);
    expect(page.querySelectorAll("[data-preference-fallback]")).toHaveLength(4);
    expect(page.getElementById("preferences-root")).not.toBeNull();
    expect(page.querySelector("[data-theme-menu]")).toBeNull();
    expect(page.querySelector(".language-select")).toBeNull();
    expect(page.querySelector(".mobile-menu__content").tagName).toBe("NAV");
    expect(page.querySelector(".mobile-preferences").closest("nav")).not.toBeNull();
  });

  test("keeps the focused product visual system original and local", () => {
    const css = readFileSync("src/styles.css", "utf8");
    const main = readFileSync("src/main.jsx", "utf8");

    expect(css).toContain(".hero-terminal");
    expect(css).toContain(".terminal-preview");
    expect(css.match(/linear-gradient/g)).toHaveLength(2);
    expect(css).toContain(".hero h1");
    expect(existsSync("public/images/og-cli-tools-landing.png")).toBe(true);
    expect(main).not.toContain("fonts.googleapis.com");
    expect(main).not.toContain("@fontsource-variable");
    expect(main).toContain("@fontsource/geist");
    expect(main).toContain("@fontsource/geist-mono");
    expect(css).not.toContain("paperclip");
  });

  test("keeps prose together without changing literal code wrapping", () => {
    const style = document.createElement("style");
    const code = document.createElement("code");
    style.textContent = readFileSync("src/styles.css", "utf8");
    document.head.append(style);
    document.body.append(code);

    expect(getComputedStyle(document.body).wordBreak).toBe("keep-all");
    expect(getComputedStyle(document.body).overflowWrap).toBe("anywhere");
    expect(getComputedStyle(code).wordBreak).toBe("normal");
    expect(getComputedStyle(code).overflowWrap).toBe("normal");

    code.remove();
    style.remove();
  });

  test("keeps the zzz content within its responsive grid column", () => {
    const style = document.createElement("style");
    const content = document.createElement("div");
    content.className = "zzz-feature__content";
    style.textContent = readFileSync("src/styles.css", "utf8");
    document.head.append(style);
    document.body.append(content);

    expect(getComputedStyle(content).width).toBe("100%");

    content.remove();
    style.remove();
  });

  test("allows analysis cards to shrink inside narrow grid columns", () => {
    const style = document.createElement("style");
    const tools = document.createElement("div");
    const card = document.createElement("article");
    tools.className = "analysis__tools";
    tools.append(card);
    style.textContent = readFileSync("src/styles.css", "utf8");
    document.head.append(style);
    document.body.append(tools);

    expect(getComputedStyle(card).minWidth).toBe("0px");

    tools.remove();
    style.remove();
  });

  test("uses the #4AF626 primary across themes and branded surfaces", () => {
    const css = readFileSync("src/styles.css", "utf8");
    const legalCss = readFileSync("public/legal.css", "utf8");
    const favicon = readFileSync("public/favicon.svg", "utf8");

    expect(css.match(/--primary: #4AF626;/g)).toHaveLength(2);
    expect(css).toContain("--accent-solid: var(--primary);");
    expect(css).toContain("color: var(--primary);");
    const legalPrimaryValues = [
      ...legalCss.matchAll(/--primary:\s*([^;]+);/g),
    ].map((match) => match[1]);
    expect(legalPrimaryValues.length).toBeGreaterThanOrEqual(2);
    expect(new Set(legalPrimaryValues)).toEqual(new Set(["#4AF626"]));
    expect(legalCss).not.toContain("--button:");
    expect(favicon).toContain('fill="#4AF626"');
  });

  test("ships legal routes and a branded route home from 404", () => {
    const privacy = renderLegal("ko", "privacy");
    const terms = renderLegal("ko", "terms");
    const notFound = renderNotFound();

    expect(privacy).toContain("개인정보 처리 안내");
    expect(privacy).toContain('content="noindex,follow"');
    expect(terms).toContain("MIT License");
    expect(notFound).toContain('href="/cli-tools/"');
    expect(notFound).toContain("페이지를 찾을 수 없습니다");
  });

  test("renders the footer wordmark as a decorative letter sequence", () => {
    const page = new DOMParser().parseFromString(renderLanding("ko"), "text/html");
    const wordmark = page.querySelector("[data-footer-wordmark]");
    const letters = [...wordmark.querySelectorAll("span")];

    expect(wordmark.getAttribute("aria-hidden")).toBe("true");
    expect(wordmark.textContent.trim()).toBe("cli-tools");
    expect(letters).toHaveLength(9);
    expect(
      letters.map((letter) =>
        letter.style.getPropertyValue("--wordmark-index"),
      ),
    ).toEqual(["0", "1", "2", "3", "4", "5", "6", "7", "8"]);
  });

  test("introduces every binary and switches to its real usage examples", async () => {
    const user = userEvent.setup();
    renderApp();

    for (const name of [
      "code-cost",
      "work-summary",
      "git-tools",
      "dev-tools",
      "zzz",
    ]) {
      expect(screen.getByRole("tab", { name: new RegExp(name) })).toBeTruthy();
    }

    await user.click(screen.getByRole("tab", { name: /zzz/ }));
    await waitFor(() => {
      expect(document.getElementById("tool-panel-zzz")).not.toBeNull();
    });
    const panel = document.getElementById("tool-panel-zzz");
    expect(panel.textContent).toContain("zzz --wait cargo test");
    expect(panel.textContent).toContain("~/.commands/{yymmdd}");
  });

  test("completes the landing argument from benefits through FAQ", () => {
    const { container } = renderApp();

    expect(
      screen.getByRole("heading", {
        name: "하루에 몇 번씩 하던 일을, 한 번의 명령으로.",
      }),
    ).toBeTruthy();
    expect(container.querySelectorAll(".benefit-grid article")).toHaveLength(4);
    const tagline = screen.getByRole("heading", {
      name: "터미널을 떠나지 않고, 분석하고 정리하고 다음 작업으로.",
    });
    const taglineWords = [...container.querySelectorAll(".tagline__word")];

    expect(tagline.textContent).toBe(
      "터미널을 떠나지 않고, 분석하고 정리하고 다음 작업으로.",
    );
    expect(taglineWords).toHaveLength(7);
    expect(taglineWords.every((word) => word.style.opacity === "")).toBe(true);
    expect(container.querySelectorAll(".workflow-list li")).toHaveLength(3);
    expect(container.querySelectorAll(".faq-list dt")).toHaveLength(6);

    const finalAction = screen.getByRole("link", { name: /설치 명령 보기/ });
    expect(finalAction.getAttribute("href")).toBe("#install");
  });

  test("uses literal spaces instead of CSS margins between tagline words", () => {
    const style = document.createElement("style");
    style.textContent = readFileSync("src/styles.css", "utf8");
    document.head.append(style);

    try {
      const { container } = renderApp();
      const firstWord = container.querySelector(".tagline__word");

      expect(Number.parseFloat(getComputedStyle(firstWord).marginRight)).toBe(0);
    } finally {
      style.remove();
    }
  });

  test("copies commands and exposes success feedback", async () => {
    renderApp();
    const copyButton = screen.getAllByRole("button", { name: /복사/ })[0];

    fireEvent.click(copyButton);

    await waitFor(() => {
      expect(screen.getAllByRole("status")[0].textContent).toContain(
        "클립보드에 복사했습니다",
      );
    });
  });

  test("shows a contextual clipboard error", async () => {
    navigator.clipboard.writeText = vi.fn().mockRejectedValueOnce(new Error("denied"));
    renderApp();

    fireEvent.click(screen.getAllByRole("button", { name: /복사/ })[0]);

    await waitFor(() => {
      expect(screen.getAllByRole("status")[0].textContent).toContain(
        "클립보드 권한",
      );
    });
  });

  test("filters dev-tools examples by category and command", async () => {
    const user = userEvent.setup();
    renderApp();

    await user.click(screen.getByRole("tab", { name: "네트워크" }));
    await user.click(screen.getByRole("button", { name: "dev-tools cert" }));

    await waitFor(() => {
      expect(document.body.textContent).toContain("dev-tools cert example.com");
    });
  });

  test("supports arrow-key navigation across dev-tools categories", async () => {
    const user = userEvent.setup();
    renderApp();

    const dataTab = screen.getByRole("tab", { name: "데이터 형식" });
    dataTab.focus();
    await user.keyboard("{ArrowRight}");

    expect(screen.getByRole("tab", { name: "ID와 보안" }).getAttribute("aria-selected")).toBe(
      "true",
    );
    expect(document.activeElement.textContent).toBe("ID와 보안");
  });

  test("makes horizontally scrollable command examples keyboard accessible", () => {
    const { container } = renderApp();
    const examples = [...container.querySelectorAll("pre")];
    const buildCommand = screen.getByRole("region", {
      name: "전체 빌드 명령 코드",
    });
    const testCommand = screen.getByRole("region", {
      name: "전체 테스트 명령 코드",
    });
    const analysisCommands = [
      "code-cost 분석 명령 코드",
      "work-summary 분석 명령 코드",
      "git-tools 분석 명령 코드",
    ].map((name) => screen.getByRole("region", { name }));

    expect(examples.length).toBeGreaterThan(0);
    expect(examples.every((example) => example.tabIndex === 0)).toBe(true);
    expect(examples.every((example) => example.getAttribute("role") === "region")).toBe(
      true,
    );
    expect(examples.every((example) => example.getAttribute("aria-label"))).toBe(true);
    expect(buildCommand.tabIndex).toBe(0);
    expect(testCommand.tabIndex).toBe(0);
    expect(analysisCommands.every((command) => command.tabIndex === 0)).toBe(true);
  });

  test("keeps visible copy free of banned dash characters", () => {
    renderApp();
    expect(document.body.textContent).not.toMatch(/[—–]/);
  });

  test("renders localized interactive content without translating commands", async () => {
    const user = userEvent.setup();

    const english = renderApp("en");
    expect(screen.getByRole("heading", { name: "Five tools. One workflow." })).toBeTruthy();
    expect(screen.getAllByText("Repository value estimate").length).toBeGreaterThan(0);
    expect(screen.getByText("Questions before you install.")).toBeTruthy();
    expect(document.body.textContent).toContain("code-cost --export report.html");
    fireEvent.click(screen.getAllByRole("button", { name: /Copy/ })[0]);
    await waitFor(() => {
      expect(screen.getAllByRole("status")[0].textContent).toContain(
        "Copied to the clipboard.",
      );
    });
    english.unmount();

    const japanese = renderApp("ja");
    expect(screen.getByRole("heading", { name: "5つのツール、一つの作業フロー。" })).toBeTruthy();
    japanese.unmount();

    renderApp("zh");
    expect(screen.getByRole("heading", { name: "五个工具，一套工作流。" })).toBeTruthy();
    await user.click(screen.getByRole("tab", { name: /zzz/ }));
    expect(document.body.textContent).toContain("zzz --wait cargo test");
  });
});
