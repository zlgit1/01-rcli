mod cli;
mod process;

pub use cli::{Base64Format, Base64Subcommand, Opts, Subcommand};
pub use process::{process_csv, process_decode, process_encode, process_genpass};
