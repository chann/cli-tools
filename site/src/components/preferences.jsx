import { createPortal } from "react-dom";
import { useEffect, useMemo, useState } from "react";
import {
  Select,
  SelectContent,
  SelectGroup,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import { useI18n } from "@/i18n/context";
import { LOCALES, localizedPath } from "@/i18n/locale";
import {
  THEME_MODES,
  applyTheme,
  readPreference,
  writePreference,
} from "@/runtime/preferences";

export function PreferenceSelect({
  ariaLabel,
  value,
  options,
  widthClass,
  onValueChange,
}) {
  return (
    <Select value={value} onValueChange={onValueChange}>
      <SelectTrigger
        aria-label={ariaLabel}
        className={`preference-select ${widthClass}`}
      >
        <SelectValue />
      </SelectTrigger>
      <SelectContent className="min-w-[136px]" position="popper" align="end">
        <SelectGroup>
          {options.map((option) => (
            <SelectItem key={option.value} value={option.value}>
              {option.label}
            </SelectItem>
          ))}
        </SelectGroup>
      </SelectContent>
    </Select>
  );
}

function PreferenceSet({
  compact,
  locale,
  languageOptions,
  themeMode,
  themeOptions,
  messages,
  onLocaleChange,
  onThemeChange,
}) {
  const languageWidth = compact ? "w-[72px]" : "w-[136px]";
  const themeWidth = compact ? "w-[104px]" : "w-[136px]";

  return (
    <div
      className={`preference-controls${compact ? "" : " preference-controls--mobile"}`}
    >
      <div className="preference-control" data-preference-kind="language">
        {!compact && <p>{messages.shell.languageLabel}</p>}
        <PreferenceSelect
          ariaLabel={messages.shell.languageLabel}
          value={locale}
          options={languageOptions}
          widthClass={languageWidth}
          onValueChange={onLocaleChange}
        />
      </div>
      <div className="preference-control" data-preference-kind="theme">
        {!compact && <p>{messages.shell.themeTitle}</p>}
        <PreferenceSelect
          ariaLabel={messages.shell.themeLabel}
          value={themeMode}
          options={themeOptions}
          widthClass={themeWidth}
          onValueChange={onThemeChange}
        />
      </div>
    </div>
  );
}

export function PreferenceControls({
  hosts,
  documentRef = window.document,
  storage = window.localStorage,
  media = window.matchMedia("(prefers-color-scheme: dark)"),
  navigate = (href) => window.location.assign(href),
}) {
  const { locale: initialLocale, messages } = useI18n();
  const [locale, setLocale] = useState(initialLocale);
  const [themeMode, setThemeMode] = useState(() => {
    const documentMode = documentRef.documentElement.dataset.themeMode;
    if (THEME_MODES.includes(documentMode)) return documentMode;
    const storedMode = readPreference(storage, "cli-tools-theme");
    return THEME_MODES.includes(storedMode) ? storedMode : "system";
  });

  const compactLanguageOptions = useMemo(
    () =>
      Object.entries(LOCALES).map(([value, metadata]) => ({
        value,
        label: metadata.shortLabel,
      })),
    [],
  );
  const fullLanguageOptions = useMemo(
    () =>
      Object.entries(LOCALES).map(([value, metadata]) => ({
        value,
        label: metadata.label,
      })),
    [],
  );
  const themeOptions = useMemo(
    () => [
      { value: "system", label: messages.shell.themeSystem },
      { value: "light", label: messages.shell.themeLight },
      { value: "dark", label: messages.shell.themeDark },
    ],
    [messages],
  );

  useEffect(() => {
    applyTheme(documentRef, themeMode, media.matches);

    const handleMediaChange = () => {
      if (themeMode === "system") {
        applyTheme(documentRef, themeMode, media.matches);
      }
    };
    media.addEventListener("change", handleMediaChange);
    return () => media.removeEventListener("change", handleMediaChange);
  }, [documentRef, media, themeMode]);

  const handleLocaleChange = (nextLocale) => {
    if (!LOCALES[nextLocale]) return;
    setLocale(nextLocale);
    writePreference(storage, "cli-tools-locale", nextLocale);
    navigate(localizedPath(nextLocale));
  };

  const handleThemeChange = (nextMode) => {
    if (!THEME_MODES.includes(nextMode)) return;
    setThemeMode(nextMode);
    writePreference(storage, "cli-tools-theme", nextMode);
    applyTheme(documentRef, nextMode, media.matches);
  };

  const desktop = (
    <PreferenceSet
      compact
      locale={locale}
      languageOptions={compactLanguageOptions}
      themeMode={themeMode}
      themeOptions={themeOptions}
      messages={messages}
      onLocaleChange={handleLocaleChange}
      onThemeChange={handleThemeChange}
    />
  );
  const mobile = (
    <PreferenceSet
      locale={locale}
      languageOptions={fullLanguageOptions}
      themeMode={themeMode}
      themeOptions={themeOptions}
      messages={messages}
      onLocaleChange={handleLocaleChange}
      onThemeChange={handleThemeChange}
    />
  );

  return (
    <>
      {hosts.desktop ? createPortal(desktop, hosts.desktop) : null}
      {hosts.mobile ? createPortal(mobile, hosts.mobile) : null}
    </>
  );
}
