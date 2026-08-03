import { fireEvent, render, screen, waitFor } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { existsSync, readFileSync } from "node:fs";
import { describe, expect, test, vi } from "vitest";
import App from "./App";

describe("cli-tools website", () => {
  test("renders the core message and responsive hero without JavaScript", () => {
    const html = readFileSync("index.html", "utf8");

    expect(html).toContain("<h1>반복 명령은 줄이고,");
    expect(html).toContain('id="theme-toggle"');
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

  test("uses the #4AF626 primary across themes and branded surfaces", () => {
    const css = readFileSync("src/styles.css", "utf8");
    const legalCss = readFileSync("public/legal.css", "utf8");
    const favicon = readFileSync("public/favicon.svg", "utf8");

    expect(css.match(/--primary: #4AF626;/g)).toHaveLength(2);
    expect(css).toContain("--accent-solid: var(--primary);");
    expect(css).toContain("color: var(--primary);");
    expect(legalCss.match(/--primary: #4AF626;/g)).toHaveLength(2);
    expect(legalCss).not.toContain("--button:");
    expect(favicon).toContain('fill="#4AF626"');
  });

  test("ships legal routes and a branded route home from 404", () => {
    const privacy = readFileSync("public/privacy.html", "utf8");
    const terms = readFileSync("public/terms.html", "utf8");
    const notFound = readFileSync("public/404.html", "utf8");

    expect(privacy).toContain("개인정보 처리 안내");
    expect(privacy).toContain('content="noindex,follow"');
    expect(terms).toContain("MIT License");
    expect(notFound).toContain('href="/cli-tools/"');
    expect(notFound).toContain("페이지를 찾을 수 없습니다");
  });

  test("introduces every binary and switches to its real usage examples", async () => {
    const user = userEvent.setup();
    render(<App />);

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
    const { container } = render(<App />);

    expect(
      screen.getByRole("heading", {
        name: "하루에 몇 번씩 하던 일을, 한 번의 명령으로.",
      }),
    ).toBeTruthy();
    expect(container.querySelectorAll(".benefit-grid article")).toHaveLength(4);
    expect(container.querySelectorAll(".tagline__word")).toHaveLength(7);
    expect(container.querySelectorAll(".workflow-list li")).toHaveLength(3);
    expect(container.querySelectorAll(".faq-list dt")).toHaveLength(6);

    const finalAction = screen.getByRole("link", { name: /설치 명령 보기/ });
    expect(finalAction.getAttribute("href")).toBe("#install");
  });

  test("copies commands and exposes success feedback", async () => {
    render(<App />);
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
    render(<App />);

    fireEvent.click(screen.getAllByRole("button", { name: /복사/ })[0]);

    await waitFor(() => {
      expect(screen.getAllByRole("status")[0].textContent).toContain(
        "클립보드 권한",
      );
    });
  });

  test("filters dev-tools examples by category and command", async () => {
    const user = userEvent.setup();
    render(<App />);

    await user.click(screen.getByRole("tab", { name: "네트워크" }));
    await user.click(screen.getByRole("button", { name: "dev-tools cert" }));

    await waitFor(() => {
      expect(document.body.textContent).toContain("dev-tools cert example.com");
    });
  });

  test("supports arrow-key navigation across dev-tools categories", async () => {
    const user = userEvent.setup();
    render(<App />);

    const dataTab = screen.getByRole("tab", { name: "데이터 형식" });
    dataTab.focus();
    await user.keyboard("{ArrowRight}");

    expect(screen.getByRole("tab", { name: "ID와 보안" }).getAttribute("aria-selected")).toBe(
      "true",
    );
    expect(document.activeElement.textContent).toBe("ID와 보안");
  });

  test("makes horizontally scrollable command examples keyboard accessible", () => {
    const { container } = render(<App />);
    const examples = [...container.querySelectorAll("pre")];

    expect(examples.length).toBeGreaterThan(0);
    expect(examples.every((example) => example.tabIndex === 0)).toBe(true);
    expect(examples.every((example) => example.getAttribute("role") === "region")).toBe(
      true,
    );
    expect(examples.every((example) => example.getAttribute("aria-label"))).toBe(true);
  });

  test("keeps visible copy free of banned dash characters", () => {
    render(<App />);
    expect(document.body.textContent).not.toMatch(/[—–]/);
  });
});
