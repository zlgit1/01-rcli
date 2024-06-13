use super::verify_path;
use clap::Parser;
use std::path::PathBuf;

#[derive(Debug, Parser)]
pub enum HttpSubcommand {
    #[command(about = "Serve a http server")]
    Serve(HttpServeOpts),
    // #[command(about = "verify a signature")]
    // Verify(TextVerifyOpts),
    // #[command(about = "generate a key")]
    // Generate(TextKeyGenerateOpts),
}

#[derive(Debug, Parser)]
pub struct HttpServeOpts {
    #[arg(short, long, value_parser = verify_path, default_value = ".")]
    pub dir: PathBuf,
    #[arg(short, long, default_value = "8080")]
    pub port: u16,
}
