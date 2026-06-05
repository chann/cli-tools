# Versioning

This workspace uses one version for every published CLI binary.

## Source of Truth

- The authoritative version is `workspace.package.version` in the root `Cargo.toml`.
- Every publishable crate must inherit it with `version.workspace = true`.
- Every installed binary must report that exact version from `env!("CARGO_PKG_VERSION")`.
- No crate-local package version may diverge from the workspace version.

## Commands Covered

The current release set is:

- `code-cost`
- `dev-tools`
- `git-tools`
- `work-summary`
- `zzz`

Each command must support `--version` and print:

```text
<command> <version>
```

For example, with workspace version `0.1.0`:

```text
zzz 0.1.0
```

## Release Tags

- Release tags use `vMAJOR.MINOR.PATCH`.
- The tag version must match `workspace.package.version`.
- Example: `v0.1.0` is valid only when `workspace.package.version = "0.1.0"`.

## Release Process

1. Update `workspace.package.version` in the root `Cargo.toml`.
2. Ensure all package manifests still inherit `version.workspace = true`.
3. Run the full test suite.
4. Create and push a matching tag:

   ```bash
   git tag v0.1.0
   git push origin v0.1.0
   ```

5. GitHub Actions builds release artifacts and creates the GitHub Release.

## Automation Rules

- CI must reject a release tag that does not match the workspace version.
- Release artifacts must be named with the version and target platform.
- Release artifacts must include all published binaries for that platform.
