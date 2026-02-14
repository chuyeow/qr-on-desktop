# Project AGENTS

## Repo defaults
- Use command-first verification:
  - `cargo fmt --all -- --check`
  - `cargo clippy --all-targets --all-features -- -D warnings`
  - `cargo check --all-features`
  - `cargo test --doc --all-features`
  - `cargo test --all-features`
  - `cargo build --release`

## Release workflow rules
- Release is tag-driven: tags matching `v*` trigger publish.
- Avoid duplicated paths in `softprops/action-gh-release.files`.
  - Example failure: `dist/**/*` plus `dist/checksums.txt` duplicates `checksums.txt`
  and can trigger `Not Found - update-a-release-asset`.

## Validation habits that prevent regressions
- For release validation, use a test tag and poll run status before checking artifacts.
- Validate the macOS artifact by:
  - downloading `qr-on-desktop-macos-x86_64` from run artifacts
  - listing archive entries
  - extracting and checking `Mach-O 64-bit` binary type
  - verifying checksum output exists and is non-empty.
