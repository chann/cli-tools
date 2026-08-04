const FOOTER_WORDMARK_VISIBILITY_THRESHOLD = 0.2;

export function initChrome({
  document = window.document,
  IntersectionObserver = document.defaultView?.IntersectionObserver,
} = {}) {
  const root = document.documentElement;
  const menuButton = document.getElementById("menu-toggle");
  const mobileMenu = document.getElementById("mobile-menu");
  const mobileLinks = mobileMenu ? [...mobileMenu.querySelectorAll("a")] : [];
  const footerWordmark = document.querySelector("[data-footer-wordmark]");
  let footerWordmarkObserver;

  const closeMenu = () => {
    if (!menuButton || !mobileMenu) return;
    root.classList.remove("menu-open");
    menuButton.setAttribute("aria-expanded", "false");
    if (menuButton.dataset.openLabel) {
      menuButton.setAttribute("aria-label", menuButton.dataset.openLabel);
    }
    mobileMenu.setAttribute("aria-hidden", "true");
    mobileMenu.setAttribute("inert", "");
  };

  const openMenu = () => {
    if (!menuButton || !mobileMenu) return;
    root.classList.add("menu-open");
    menuButton.setAttribute("aria-expanded", "true");
    if (menuButton.dataset.closeLabel) {
      menuButton.setAttribute("aria-label", menuButton.dataset.closeLabel);
    }
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

  if (footerWordmark) {
    if (typeof IntersectionObserver === "function") {
      footerWordmarkObserver = new IntersectionObserver(
        (entries) => {
          const wordmarkEntry = entries.find(
            (entry) => entry.target === footerWordmark,
          );
          if (!wordmarkEntry) return;

          footerWordmark.toggleAttribute(
            "data-visible",
            wordmarkEntry.isIntersecting &&
              wordmarkEntry.intersectionRatio >=
                FOOTER_WORDMARK_VISIBILITY_THRESHOLD,
          );
        },
        { threshold: FOOTER_WORDMARK_VISIBILITY_THRESHOLD },
      );
      footerWordmarkObserver.observe(footerWordmark);
    } else {
      footerWordmark.setAttribute("data-visible", "");
    }
  }

  return () => {
    menuButton?.removeEventListener("click", handleMenuClick);
    mobileLinks.forEach((link) => link.removeEventListener("click", closeMenu));
    document.removeEventListener("keydown", handleKeydown);
    footerWordmarkObserver?.disconnect();
  };
}
