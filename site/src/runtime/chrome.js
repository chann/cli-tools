export const THEME_MODES = ["system", "light", "dark"];

export function resolveTheme(mode, prefersDark) {
  if (mode === "dark") return "dark";
  if (mode === "light") return "light";
  return prefersDark ? "dark" : "light";
}

function readStoredTheme(storage) {
  try {
    const stored = storage.getItem("cli-tools-theme");
    return THEME_MODES.includes(stored) ? stored : null;
  } catch {
    return null;
  }
}

function writeStoredTheme(storage, mode) {
  try {
    storage.setItem("cli-tools-theme", mode);
  } catch {
    // The selected theme still applies for this visit.
  }
}

export function initChrome({
  document = window.document,
  storage = window.localStorage,
  media = window.matchMedia("(prefers-color-scheme: dark)"),
} = {}) {
  const root = document.documentElement;
  const themeButtons = [...document.querySelectorAll("[data-theme-option]")];
  const initialMode = THEME_MODES.includes(root.dataset.themeMode)
    ? root.dataset.themeMode
    : readStoredTheme(storage) || "system";
  let mode = initialMode;

  const applyTheme = (nextMode, persist = true) => {
    mode = THEME_MODES.includes(nextMode) ? nextMode : "system";
    root.dataset.themeMode = mode;
    root.dataset.theme = resolveTheme(mode, media.matches);
    themeButtons.forEach((button) => {
      button.setAttribute(
        "aria-pressed",
        String(button.dataset.themeOption === mode),
      );
    });
    if (persist) writeStoredTheme(storage, mode);
  };

  const themeListeners = themeButtons.map((button) => {
    const listener = () => applyTheme(button.dataset.themeOption);
    button.addEventListener("click", listener);
    return [button, listener];
  });

  const handleMediaChange = () => {
    if (mode === "system") applyTheme(mode, false);
  };
  media.addEventListener("change", handleMediaChange);
  applyTheme(mode, false);

  const menuButton = document.getElementById("menu-toggle");
  const mobileMenu = document.getElementById("mobile-menu");
  const mobileLinks = mobileMenu ? [...mobileMenu.querySelectorAll("a")] : [];

  const closeMenu = () => {
    if (!menuButton || !mobileMenu) return;
    root.classList.remove("menu-open");
    menuButton.setAttribute("aria-expanded", "false");
    menuButton.setAttribute(
      "aria-label",
      menuButton.dataset.openLabel || "메뉴 열기",
    );
    mobileMenu.setAttribute("aria-hidden", "true");
    mobileMenu.setAttribute("inert", "");
  };

  const openMenu = () => {
    if (!menuButton || !mobileMenu) return;
    root.classList.add("menu-open");
    menuButton.setAttribute("aria-expanded", "true");
    menuButton.setAttribute(
      "aria-label",
      menuButton.dataset.closeLabel || "메뉴 닫기",
    );
    mobileMenu.setAttribute("aria-hidden", "false");
    mobileMenu.removeAttribute("inert");
  };

  const handleMenuClick = () => {
    if (menuButton?.getAttribute("aria-expanded") === "true") {
      closeMenu();
    } else {
      openMenu();
    }
  };

  const handleKeydown = (event) => {
    if (
      event.key === "Escape" &&
      menuButton?.getAttribute("aria-expanded") === "true"
    ) {
      closeMenu();
      menuButton.focus();
    }
  };

  menuButton?.addEventListener("click", handleMenuClick);
  mobileLinks.forEach((link) => link.addEventListener("click", closeMenu));
  document.addEventListener("keydown", handleKeydown);

  return () => {
    themeListeners.forEach(([button, listener]) => {
      button.removeEventListener("click", listener);
    });
    media.removeEventListener("change", handleMediaChange);
    menuButton?.removeEventListener("click", handleMenuClick);
    mobileLinks.forEach((link) => link.removeEventListener("click", closeMenu));
    document.removeEventListener("keydown", handleKeydown);
  };
}
