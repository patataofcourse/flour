use clap::{Parser, Subcommand};
use flour::BCCAD;
use std::{
    fs::File,
    io::{Read, Result, Write},
    path::PathBuf,
};

#[derive(Parser)]
#[clap(
    author,
    version,
    about = "Serializes and deserializes BCCAD files to and from JSON"
)]
struct Cli {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Convert a BCCAD file into a manually editable JSON file
    Serialize {
        #[clap(parse(from_os_str))]
        /// The BCCAD file to convert
        bccad: PathBuf,
        #[clap(parse(from_os_str))]
        /// Location of the JSON file to export (optional)
        json: Option<PathBuf>,
    },
    /// Convert a JSON file exported by flour back into a BCCAD
    Deserialize {
        #[clap(parse(from_os_str))]
        /// The JSON file to convert
        json: PathBuf,
        #[clap(parse(from_os_str))]
        /// Location of the BCCAD file to export (optional)
        bccad: Option<PathBuf>,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Command::Serialize { bccad, json } => {
            let json = match json {
                Some(c) => c,
                None => {
                    let mut p = bccad.clone();
                    p.set_extension("json");
                    p
                }
            };
            let mut in_file = File::open(&bccad)?;
            let mut out_file = File::create(&json)?;
            let bccad_ = BCCAD::from_bccad(&mut in_file)?;
            let json_ = bccad_.to_json()?;
            writeln!(out_file, "{}", json_)?;
            println!(
                "Serialized {:?} to {:?}",
                bccad.into_os_string(),
                json.into_os_string()
            );
        }
        Command::Deserialize { json, bccad } => {
            let bccad = match bccad {
                Some(c) => c,
                None => {
                    let mut p = json.clone();
                    p.set_extension("bccad");
                    p
                }
            };
            let mut in_file = File::open(&json)?;
            let mut out_file = File::create(&bccad)?;
            let mut json_ = String::new();
            in_file.read_to_string(&mut json_)?;
            let bccad_ = BCCAD::from_json(&json_)?;
            bccad_.to_bccad(&mut out_file)?;
            println!(
                "Deserialized {:?} to {:?}",
                json.into_os_string(),
                bccad.into_os_string()
            );
        }
    }
    Ok(())
}

fn _main() -> Result<()> {
    let mut f = File::open("../bread/build/libs/agb_tap.bccad")?;
    let bccad = BCCAD::from_bccad(&mut f)?;
    let bccad_json = bccad.to_json()?;
    drop(f);
    let mut f = File::create("agb_tap.bccad")?;
    BCCAD::from_json(&bccad_json)?.to_bccad(&mut f)
}
