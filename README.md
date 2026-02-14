# qr-on-desktop

Read QR codes from image files and screenshots in the clipboard.
One command: screenshot a QR, run `qr-on-desktop`, get the decoded result.

## Why this exists

On macOS there is no built-in desktop workflow to convert a captured QR code into its decoded payload.  
This tool removes the phone handoff: screenshot or copy the QR in app (Shottr/CleanShot), then run one command to get the URL/text immediately.

## Requirements

- macOS / Linux / Windows
- Rust toolchain

## Build

```bash
cargo build
```

```bash
cargo build --release
```

## Install

```bash
cargo install --path .
```

## Usage

Show help:

```bash
cargo run -- --help
qr-on-desktop --help
```

Decode one or more image files:

```bash
cargo run -- ./screenshot.png
cargo run -- ./a.png ./b.png
```

Decode from clipboard:

```bash
cargo run -- --clipboard
```

No arguments defaults to clipboard:

```bash
cargo run --
```

## Project structure (canonical split)

- `src/main.rs` is the shell: parse args and print results
- `src/cli.rs` defines CLI arguments via `clap`
- `src/lib.rs` orchestrates input selection and returns decode outputs
- `src/io.rs` holds side effects (file load, clipboard read)
- `src/decoder.rs` is pure QR decode logic
- `src/fixtures.rs` is shared fixture loading/parsing
- `tests/fixture_decode.rs` is the integration test suite

## Test workflow

Generate fixture images from shared cases:

```bash
cargo run --example generate_qr_fixture
```

Run fixture and legacy smoke tests:

```bash
cargo test --test fixture_decode
```

Run all quality gates:

```bash
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test
```

### Enable local git hooks

Enable the repository-local hook path once:

```bash
git config core.hooksPath .githooks
```

This runs formatter and linter checks before every commit.

## Fixture files

- Generated fixture set path: `tests/fixtures/*.png`
- Additional smoke fixture: `tests/fixtures/qr-on-desktop-sample.png`

`tests/fixtures/cases.tsv` is the source of truth for fixture payloads.

## Notes

- Supports formats handled by the `image` crate (PNG, JPG/JPEG, BMP, GIF, etc.).
- Clipboard mode requires screenshot/image data on clipboard.
- If no payload is found in an image, the app reports no QR detected.
