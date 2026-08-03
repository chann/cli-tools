import { render, screen } from "@testing-library/react";
import { describe, expect, test } from "vitest";
import { I18nProvider, useI18n } from "./context";

function Probe() {
  const { locale, messages } = useI18n();
  return <p>{`${locale}:${messages.hero.title[0]}`}</p>;
}

describe("I18nProvider", () => {
  test("provides the requested locale before the first render", () => {
    render(
      <I18nProvider locale="en">
        <Probe />
      </I18nProvider>,
    );

    expect(screen.getByText("en:Cut the repetitive commands.")).toBeTruthy();
  });

  test("fails clearly when a consumer is outside the provider", () => {
    expect(() => render(<Probe />)).toThrow("useI18n must be used within I18nProvider");
  });
});
