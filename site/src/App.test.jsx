import { fireEvent, render, screen, waitFor } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { describe, expect, test, vi } from "vitest";
import App from "./App";

describe("cli-tools website", () => {
  test("introduces every binary and switches to its real usage examples", async () => {
    const user = userEvent.setup();
    render(<App />);

    expect(screen.getByRole("heading", { level: 1 }).textContent).toContain(
      "터미널 일을",
    );

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

  test("cycles the manual theme while preserving a single page theme", async () => {
    const user = userEvent.setup();
    render(<App />);

    const themeButton = screen.getByRole("button", { name: /테마: 시스템/ });
    await user.click(themeButton);

    expect(document.documentElement.dataset.theme).toBe("light");
    expect(document.documentElement.dataset.themeMode).toBe("light");
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

  test("keeps visible copy free of banned dash characters", () => {
    render(<App />);
    expect(document.body.textContent).not.toMatch(/[—–]/);
  });
});
