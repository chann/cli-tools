## Deploy Configuration (configured by /setup-deploy)

- Platform: GitHub Pages via GitHub Actions
- Production URL: https://chann.github.io/cli-tools/
- Deploy workflow: .github/workflows/pages.yml
- Deploy status command: gh run list --workflow pages.yml --limit 1
- Merge method: direct push to main
- Project type: Rust CLI collection with a Vite website
- Post-deploy health check: https://chann.github.io/cli-tools/

### Custom deploy hooks

- Pre-merge: cd site && pnpm check
- Deploy trigger: automatic on main when the website or Pages workflow changes
- Deploy status: gh run watch the latest Pages workflow run
- Health check: curl -fsS https://chann.github.io/cli-tools/
