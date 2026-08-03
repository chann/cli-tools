export const THEME_MODES = ["system", "light", "dark"];

export function resolveTheme(mode, prefersDark) {
  if (mode === "dark") return "dark";
  if (mode === "light") return "light";
  return prefersDark ? "dark" : "light";
}

export function readPreference(storage, key) {
  try {
    return storage.getItem(key);
  } catch {
    return null;
  }
}

export function writePreference(storage, key, value) {
  try {
    storage.setItem(key, value);
  } catch {
    // The current UI state and URL navigation still work for this visit.
  }
}

export function applyTheme(documentRef, mode, prefersDark) {
  const safeMode = THEME_MODES.includes(mode) ? mode : "system";
  documentRef.documentElement.dataset.themeMode = safeMode;
  documentRef.documentElement.dataset.theme = resolveTheme(safeMode, prefersDark);
}
