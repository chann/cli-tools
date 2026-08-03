import React from "react";
import ReactDOM from "react-dom/client";
import "@fontsource/geist/latin-400.css";
import "@fontsource/geist/latin-500.css";
import "@fontsource/geist/latin-600.css";
import "@fontsource/geist/latin-700.css";
import "@fontsource/geist-mono/latin-400.css";
import "@fontsource/geist-mono/latin-500.css";
import App from "./App";
import { I18nProvider } from "./i18n/context";
import { localeFromPath } from "./i18n/locale";
import { initChrome } from "./runtime/chrome";
import "./styles.css";

initChrome();
const locale = localeFromPath(window.location.pathname);

ReactDOM.createRoot(document.getElementById("root")).render(
  <React.StrictMode>
    <I18nProvider locale={locale}>
      <App />
    </I18nProvider>
  </React.StrictMode>,
);
