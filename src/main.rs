use clap::Parser;
use rcli::{
    process_csv, process_decode, process_encode, process_genpass, Base64Subcommand, Opts,
    Subcommand,
};

fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();
    match opts.cmd {
        Subcommand::Csv(opts) => {
            let output = if let Some(output) = opts.output {
                output.clone()
            } else {
                format!("output.{}", opts.format)
            };
            process_csv(&opts.input, output, opts.format)?;
        }
        Subcommand::Genpass(opts) => {
            let password = process_genpass(
                opts.length,
                opts.numbers,
                opts.uppercase,
                opts.lowercase,
                opts.symbols,
            )?;
            println!("{}", password);
        }
        Subcommand::Base64(subcommand) => match subcommand {
            Base64Subcommand::Encode(opts) => {
                process_encode(&opts.input, opts.format)?;
            }
            Base64Subcommand::Decode(opts) => {
                process_decode(&opts.input, opts.format)?;
            }
        },
    }
    Ok(())
}
