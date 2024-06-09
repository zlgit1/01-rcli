mod base64;
mod csv;
mod genpass;
mod text;
use self::{csv::CsvOpts, genpass::GenpassOpts};
use clap::Parser;
use std::path::Path;

pub use self::{
    base64::{Base64Format, Base64Subcommand},
    csv::OutputFormat,
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
}
fn verify_input_file(filename: &str) -> Result<String, &'static str> {
    if filename == "-" || Path::new(filename).exists() {
        Ok(filename.into())
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
