mod base64;
mod csv;
mod genpass;
mod http;
mod text;

use self::{csv::CsvOpts, genpass::GenpassOpts};
use clap::Parser;
use std::path::{Path, PathBuf};

pub use self::{
    base64::{Base64Format, Base64Subcommand},
    csv::OutputFormat,
    http::HttpSubcommand,
    text::{TextSignFormat, TextSubcommand},
};

#[derive(Debug, Parser)]
#[command(name = "rust-cli", version, about, long_about = None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: Subcommand,
}
#[derive(Debug, Parser)]
pub enum Subcommand {
    #[command(name = "csv", about = "Show CSV or convert CSV to other formats")]
    Csv(CsvOpts),
    #[command(name = "genpass", about = "Generate a random password")]
    Genpass(GenpassOpts),
    #[command(subcommand)]
    Base64(Base64Subcommand),
    #[command(subcommand)]
    Text(TextSubcommand),
    #[command(subcommand)]
    Http(HttpSubcommand),
}
fn verify_input_file(filename: &str) -> Result<String, &'static str> {
    if filename == "-" || Path::new(filename).exists() {
        Ok(filename.into())
    } else {
        Err("File does not exist")
    }
}

fn verify_path(path: &str) -> Result<PathBuf, &'static str> {
    let p = Path::new(path);
    if p.exists() && p.is_dir() {
        Ok(path.into())
    } else {
        Err("File does not exist")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_input_file_test() {
        assert_eq!(verify_input_file("-"), Ok("-".into()));
        assert_eq!(verify_input_file("*"), Err("File does not exist"));
        assert_eq!(verify_input_file("not-exist"), Err("File does not exist"));
    }
}
