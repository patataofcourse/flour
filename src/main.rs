use clap::{Parser, Subcommand};
use flour::BCCAD;
use std::{
    fs::File,
    io::{Result, Write},
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
    command: Option<Command>,
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
        Some(Command::Serialize { bccad, json }) => {
            let json = match json {
                Some(c) => c,
                None => {
                    let mut p = bccad.clone();
                    p.set_extension(".json");
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
        Some(Command::Deserialize { json, bccad }) => {}
        None => {}
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
