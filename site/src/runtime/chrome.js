export function initChrome({ document = window.document } = {}) {
  const root = document.documentElement;
  const menuButton = document.getElementById("menu-toggle");
  const mobileMenu = document.getElementById("mobile-menu");
  const mobileLinks = mobileMenu ? [...mobileMenu.querySelectorAll("a")] : [];

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

  return () => {
    menuButton?.removeEventListener("click", handleMenuClick);
    mobileLinks.forEach((link) => link.removeEventListener("click", closeMenu));
    document.removeEventListener("keydown", handleKeydown);
  };
}
