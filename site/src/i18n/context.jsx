import { createContext, useContext, useMemo } from "react";
import { getMessages } from "./catalogs";
import { LOCALES } from "./locale";

const I18nContext = createContext(null);

export function I18nProvider({ locale = "ko", children }) {
  const activeLocale = LOCALES[locale] ? locale : "ko";
  const value = useMemo(
    () => ({ locale: activeLocale, messages: getMessages(activeLocale) }),
    [activeLocale],
  );

  return <I18nContext.Provider value={value}>{children}</I18nContext.Provider>;
}

export function useI18n() {
  const value = useContext(I18nContext);
  if (!value) throw new Error("useI18n must be used within I18nProvider");
  return value;
}
