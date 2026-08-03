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
  const themeMenus = [...document.querySelectorAll("[data-theme-menu]")];
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
        "aria-checked",
        String(button.dataset.themeOption === mode),
      );
    });
    const activeLabel = themeButtons.find(
      (button) => button.dataset.themeOption === mode,
    )?.textContent.trim();
    document.querySelectorAll("[data-theme-current]").forEach((label) => {
      if (activeLabel) label.textContent = activeLabel;
    });
    if (persist) writeStoredTheme(storage, mode);
  };

  const closeThemeMenu = (themeMenu, restoreFocus = false) => {
    const trigger = themeMenu.querySelector("[data-theme-trigger]");
    const content = themeMenu.querySelector('[role="menu"]');
    if (!trigger || !content) return;
    trigger.setAttribute("aria-expanded", "false");
    content.hidden = true;
    if (restoreFocus) trigger.focus();
  };

  const closeOtherThemeMenus = (currentMenu) => {
    themeMenus.forEach((themeMenu) => {
      if (themeMenu !== currentMenu) closeThemeMenu(themeMenu);
    });
  };

  const themeListeners = themeButtons.map((button) => {
    const listener = () => {
      applyTheme(button.dataset.themeOption);
      const themeMenu = button.closest("[data-theme-menu]");
      if (themeMenu) closeThemeMenu(themeMenu, true);
    };
    button.addEventListener("click", listener);
    return [button, listener];
  });

  const themeMenuListeners = themeMenus.flatMap((themeMenu) => {
    const trigger = themeMenu.querySelector("[data-theme-trigger]");
    const content = themeMenu.querySelector('[role="menu"]');
    const items = [...themeMenu.querySelectorAll('[role="menuitemradio"]')];
    if (!trigger || !content || items.length === 0) return [];

    const openMenu = () => {
      closeOtherThemeMenus(themeMenu);
      trigger.setAttribute("aria-expanded", "true");
      content.hidden = false;
      const selected = items.find((item) => item.dataset.themeOption === mode);
      (selected || items[0]).focus();
    };
    const toggleMenu = () => {
      if (trigger.getAttribute("aria-expanded") === "true") {
        closeThemeMenu(themeMenu, true);
      } else {
        openMenu();
      }
    };
    const handleTriggerKeydown = (event) => {
      if (event.key === "ArrowDown" || event.key === "ArrowUp") {
        event.preventDefault();
        openMenu();
        if (event.key === "ArrowUp") items.at(-1)?.focus();
      }
    };
    const handleItemKeydown = (event) => {
      const index = items.indexOf(event.currentTarget);
      let nextIndex = null;
      if (event.key === "ArrowDown") nextIndex = (index + 1) % items.length;
      if (event.key === "ArrowUp") nextIndex = (index - 1 + items.length) % items.length;
      if (event.key === "Home") nextIndex = 0;
      if (event.key === "End") nextIndex = items.length - 1;
      if (nextIndex !== null) {
        event.preventDefault();
        items[nextIndex].focus();
      }
      if (event.key === "Escape") {
        event.preventDefault();
        closeThemeMenu(themeMenu, true);
      }
    };

    trigger.addEventListener("click", toggleMenu);
    trigger.addEventListener("keydown", handleTriggerKeydown);
    items.forEach((item) => item.addEventListener("keydown", handleItemKeydown));

    return [
      [trigger, "click", toggleMenu],
      [trigger, "keydown", handleTriggerKeydown],
      ...items.map((item) => [item, "keydown", handleItemKeydown]),
    ];
  });

  const handleOutsideThemeClick = (event) => {
    themeMenus.forEach((themeMenu) => {
      if (!themeMenu.contains(event.target)) closeThemeMenu(themeMenu);
    });
  };
  document.addEventListener("click", handleOutsideThemeClick);

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
    themeMenuListeners.forEach(([element, eventName, listener]) => {
      element.removeEventListener(eventName, listener);
    });
    document.removeEventListener("click", handleOutsideThemeClick);
    media.removeEventListener("change", handleMediaChange);
    menuButton?.removeEventListener("click", handleMenuClick);
    mobileLinks.forEach((link) => link.removeEventListener("click", closeMenu));
    document.removeEventListener("keydown", handleKeydown);
  };
}
