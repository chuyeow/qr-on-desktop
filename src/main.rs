use anyhow::Result;
use clap::Parser;
use qr_on_desktop::{Cli, decode_inputs};
use std::process;

fn main() {
    let cli = Cli::parse();
    if let Err(err) = run(cli) {
        eprintln!("{err:#}");
        process::exit(1);
    }
}

fn run(cli: Cli) -> Result<()> {
    let outputs = decode_inputs(&cli);
    let mut found_any = false;
    let mut had_errors = false;
    let should_prefix = outputs.len() > 1;

    for output in outputs {
        if let Some(error) = output.error {
            had_errors = true;
            eprintln!("Failed to read `{}`: {error}", output.source);
            continue;
        }

        if output.values.is_empty() {
            eprintln!("No QR code found: {}", output.source);
            continue;
        }

        found_any = true;
        for value in output.values {
            if should_prefix {
                println!("{}: {}", output.source, value);
            } else {
                println!("{value}");
            }
        }
    }

    if had_errors && !found_any {
        process::exit(1);
    }
    Ok(())
}
