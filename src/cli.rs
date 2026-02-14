use clap::Parser;
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(name = "qr-on-desktop")]
#[command(about = "Decode QR codes from screenshot images", version)]
pub struct Cli {
    #[arg(value_name = "PATH")]
    pub paths: Vec<PathBuf>,

    #[arg(short, long)]
    pub clipboard: bool,
}

impl Cli {
    pub fn should_use_clipboard(&self) -> bool {
        self.clipboard || self.paths.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::Cli;
    use clap::Parser;

    #[test]
    fn defaults_to_clipboard_without_paths() {
        let cli = Cli::parse_from(["qr-on-desktop"]);
        assert!(cli.should_use_clipboard());
    }

    #[test]
    fn uses_paths_when_provided() {
        let cli = Cli::parse_from(["qr-on-desktop", "one.png", "two.png"]);
        assert_eq!(cli.paths.len(), 2);
        assert!(!cli.should_use_clipboard());
    }
}
