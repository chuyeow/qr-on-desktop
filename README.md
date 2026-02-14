# qr-on-desktop

Read QR codes from image files and screenshots in the clipboard.
One command: screenshot a QR, run `qr-on-desktop`, get the decoded result.

## Why this exists

On macOS there is no built-in desktop flow to convert a captured QR into its payload.
This tool removes the phone handoff: screenshot or copy the QR in Shottr/CleanShot, then run one command to get the URL/text immediately.

## Usage

Run the binary with no args (defaults to clipboard):

```bash
qr-on-desktop
```

Run against one or more image files:

```bash
qr-on-desktop screenshot.png
qr-on-desktop a.png b.png
```

Show help:

```bash
qr-on-desktop --help
```

## Install

From source (requires Rust):

```bash
cargo install --path .
```

## Typical macOS workflow

1. In Shottr/CleanShot, capture a screenshot of the QR code to clipboard.
2. Run:

```bash
qr-on-desktop
```

3. Read the output URLs/text in your terminal.

## Release behavior

Tag to release:

```text
vX.Y.Z
```

Example:

```bash
git tag v0.2.0
git push origin v0.2.0
```

## For developers

### Local quality gates

```bash
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo check --all-features
cargo test --doc --all-features
cargo test --all-features
```

### Enable local git hooks

Enable repository-local hook path once:

```bash
git config core.hooksPath .githooks
```

This runs formatter and linter checks before every commit.
