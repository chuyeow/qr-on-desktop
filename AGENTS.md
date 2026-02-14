# Project AGENTS

Purpose:
- Capture repository-specific guidance that reduces friction from repeated work.
- Keep actions consistent and predictable across local and CI workflows.

## Repo defaults
- Use command-first verification:
  - `cargo fmt --all -- --check`
  - `cargo clippy --all-targets --all-features -- -D warnings`
  - `cargo check --all-features`
  - `cargo test --doc --all-features`
  - `cargo test --all-features`
  - `cargo build --release`
- Always keep `.github/workflows` as source of truth for canonical CI/release behavior.

## Release workflow rules
- Release is tag-driven: tags matching `v*` trigger publish.
- Avoid duplicated paths in `softprops/action-gh-release.files`.
  - Example failure: `dist/**/*` plus `dist/checksums.txt` duplicates `checksums.txt`
  and can trigger `Not Found - update-a-release-asset`.
- Keep artifacts deterministic:
  - Linux/macOS: `.tar.gz`
  - Windows: `.zip`
  - Include one generated `checksums.txt` file alongside artifacts.

## Validation habits that prevent regressions
- For release validation, use a test tag and poll run status before checking artifacts.
- Validate the macOS artifact by:
  - downloading `qr-on-desktop-macos-x86_64` from run artifacts
  - listing archive entries
  - extracting and checking `Mach-O 64-bit` binary type
  - verifying checksum output exists and is non-empty.

## Git/CLI conventions
- Prefer lightweight, explicit commit messages.
- Keep commits focused to one functional change where possible.
