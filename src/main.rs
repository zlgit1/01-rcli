use std::fs;

use clap::Parser;
use rcli::{
    process_csv, process_decode, process_encode, process_generate, process_genpass,
    process_text_sign, process_text_verify, Base64Subcommand, Opts, Subcommand, TextSubcommand,
};
use zxcvbn::zxcvbn;

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
            let estimate = zxcvbn(&password, &[]);
            eprintln!("Password strength: {}", estimate.score());
        }
        Subcommand::Base64(subcommand) => match subcommand {
            Base64Subcommand::Encode(opts) => {
                let encoded = process_encode(&opts.input, opts.format)?;
                println!("{}", encoded);
            }
            Base64Subcommand::Decode(opts) => {
                let decoded = process_decode(&opts.input, opts.format)?;
                let decoded = String::from_utf8(decoded)?;
                println!("{}", decoded);
            }
        },
        Subcommand::Text(subcommand) => match subcommand {
            TextSubcommand::Sign(opts) => {
                let signed = process_text_sign(&opts.input, &opts.key, opts.format)?;
                println!("{}", signed);
            }
            TextSubcommand::Verify(opts) => {
                let verified =
                    process_text_verify(&opts.input, &opts.key, opts.format, &opts.signature)?;
                println!("{}", verified);
            }
            TextSubcommand::Generate(opts) => {
                let key = process_generate(opts.format)?;
                match opts.format {
                    rcli::TextSignFormat::Blake3 => {
                        let name = opts.output.join("blake3.txt");
                        fs::write(name, &key[0])?;
                    }
                    rcli::TextSignFormat::Ed25519 => {
                        let name = &opts.output;
                        fs::write(name.join("ed25519.sk"), &key[0])?;
                        fs::write(name.join("ed25519.pk"), &key[1])?;
                    }
                }
            }
        },
    }
    Ok(())
}
