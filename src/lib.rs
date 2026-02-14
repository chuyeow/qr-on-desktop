use anyhow::{Context, Result};

pub mod cli;
pub mod decoder;
pub mod fixtures;
pub mod io;

pub use cli::Cli;

#[derive(Debug)]
pub struct DecodeOutput {
    pub source: String,
    pub values: Vec<String>,
    pub error: Option<String>,
}

pub fn decode_inputs(cli: &Cli) -> Vec<DecodeOutput> {
    let mut results = Vec::new();
    let uses_clipboard = cli.clipboard || cli.paths.is_empty();

    if uses_clipboard {
        let source = if cli.clipboard {
            "clipboard".to_string()
        } else {
            "clipboard (default)".to_string()
        };
        results.push(decode_from_source(&source, || decode_clipboard()));
        return results;
    }

    for path in &cli.paths {
        let source = path.display().to_string();
        let path_buf = path.clone();
        results.push(decode_from_source(&source, move || decode_file(path_buf.as_path())));
    }

    results
}

pub fn decode_file(path: &std::path::Path) -> Result<Vec<String>> {
    let image = io::load_file_to_gray(path).context("failed to load image from disk")?;
    Ok(decoder::decode_qr_from_gray_image(image))
}

pub fn decode_clipboard() -> Result<Vec<String>> {
    let image = io::load_clipboard_to_gray().context("failed to load image from clipboard")?;
    Ok(decoder::decode_qr_from_gray_image(image))
}

fn decode_from_source(
    source: &str,
    decoder_fn: impl FnOnce() -> Result<Vec<String>>,
) -> DecodeOutput {
    match decoder_fn() {
        Ok(values) => DecodeOutput {
            source: source.to_string(),
            values,
            error: None,
        },
        Err(error) => DecodeOutput {
            source: source.to_string(),
            values: Vec::new(),
            error: Some(error.to_string()),
        },
    }
}
