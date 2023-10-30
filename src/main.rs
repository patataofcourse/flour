use clap::{Parser, Subcommand};
use flour::{
    bxcad::{
        self,
        qol::{IndexizedBCCAD, IndexizedBRCAD},
        BXCADType, BXCADWrapper, BXCAD,
    },
    error::{Error, Result},
    BCCAD, BRCAD,
};
use json_comments::{CommentSettings, StripComments};
use serde_json::Value;
use std::{
    fs::File,
    io::{Read, Write},
    path::PathBuf,
};

#[derive(Parser, Debug)]
#[clap(
    author,
    version,
    about = "Serializes and deserializes BCCAD/BRCAD files to and from JSON"
)]
struct Cli {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Convert a BCCAD file into a manually editable JSON file
    #[clap(aliases = &["serialise", "s"])]
    Serialize {
        #[clap(parse(from_os_str))]
        /// The B_CAD file to convert
        bxcad: PathBuf,
        #[clap(parse(from_os_str))]
        /// Location of the JSON file to export (optional)
        json: Option<PathBuf>,

        /// File is a BCCAD
        #[clap(short = 'c', long)]
        is_bccad: bool,
        /// File is a BRCAD
        #[clap(short = 'r', long, conflicts_with = "is-bccad")]
        is_brcad: bool,
        /// Automatically detect whether the file is a BRCAD or a BCCAD (default)
        #[clap(
            short = 'a',
            long,
            conflicts_with = "is-brcad",
            conflicts_with = "is-bccad"
        )]
        auto: bool,

        /// (BRCAD only) Adds labels from label file
        #[clap(short, long, parse(from_os_str))]
        labels: Option<PathBuf>,
        /// Display indexes next to the sprites
        #[clap(short, long, alias = "indexise")]
        indexize: bool,
    },
    /// Convert a JSON file exported by flour back into a BCCAD
    #[clap(aliases = &["deserialise", "d"])]
    Deserialize {
        /// The JSON file to convert
        #[clap(parse(from_os_str))]
        json: PathBuf,
        /// Location of the B_CAD file to export (optional)
        #[clap(parse(from_os_str))]
        bxcad: Option<PathBuf>,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Command::Serialize {
            bxcad,
            json,
            is_bccad,
            is_brcad,
            labels,
            indexize,
            ..
        } => {
            let json = match json {
                Some(c) => c,
                None => {
                    let mut p = bxcad.clone();
                    p.set_extension("json");
                    p
                }
            };

            let mut in_file = File::open(&bxcad)?;

            let bxcad_type = if is_bccad {
                BXCADType::BCCAD
            } else if is_brcad {
                BXCADType::BRCAD
            } else {
                bxcad::get_bxcad_type(&mut in_file)?.ok_or(Error::NotBXCAD)?
            };

            if labels.is_some() && bxcad_type != BXCADType::BRCAD {
                Err(Error::LabelsOnNotBRCAD)?
            }

            let json_ = match bxcad_type {
                BXCADType::BCCAD => {
                    let bccad = BCCAD::from_binary(&mut in_file)?;
                    if indexize {
                        let wrapper = BXCADWrapper::from_bxcad_indexize(bccad);
                        serde_json::to_string_pretty(&wrapper)
                    } else {
                        let wrapper = BXCADWrapper::from_bxcad(bccad);
                        serde_json::to_string_pretty(&wrapper)
                    }
                }
                BXCADType::BRCAD => {
                    let mut brcad = BRCAD::from_binary(&mut in_file)?;
                    if let Some(c) = labels {
                        let mut labels_file = File::open(c)?;
                        brcad.apply_labels(&mut labels_file)?;
                    }
                    if indexize {
                        let wrapper = BXCADWrapper::from_bxcad_indexize(brcad);

                        serde_json::to_string_pretty(&wrapper)
                    } else {
                        let wrapper = BXCADWrapper::from_bxcad(brcad);

                        serde_json::to_string_pretty(&wrapper)
                    }
                }
                //  BXCADType::Custom(_) => Err(Error::NonImplementedFeature(
                //      "custom BXCAD types".to_string(),
                //  ))?,
                c => Err(Error::NonImplementedFeature(format!("BXCAD type {:?}", c)))?,
            }?;

            let mut out_file = File::create(&json)?;

            writeln!(out_file, "{}", json_)?;
            println!(
                "Serialized {:?} to {:?}",
                bxcad.into_os_string(),
                json.into_os_string()
            );
        }
        Command::Deserialize { json, bxcad } => {
            let in_file = File::open(&json)?;
            let mut stripped = StripComments::with_settings(CommentSettings::c_style(), in_file);

            let mut json_ = String::new();
            stripped.read_to_string(&mut json_)?;
            let value_wrapper: Value = serde_json::from_str(&json_)?;

            let Some(bxcad_type) = value_wrapper.get("bxcad_type") else {
                Err(Error::NotFlour)?
            };
            let Some(bxcad_type) = bxcad_type.as_str() else {
                Err(Error::NotFlour)?
            };
            let bxcad_type: BXCADType = serde_json::from_str(&format!("\"{}\"", bxcad_type))?;

            let bxcad = match bxcad {
                Some(c) => c,
                None => {
                    let mut p = json.clone();
                    p.set_extension(match &bxcad_type {
                        BXCADType::BCCAD => "bccad",
                        BXCADType::BRCAD => "brcad",
                        //  BXCADType::Custom(_) => todo!(),
                        _ => unimplemented!(),
                    });
                    p
                }
            };

            let indexized = if let Some(bxcad_type) = value_wrapper.get("indexize") {
                let Some(c) = bxcad_type.as_bool() else {
                    Err(Error::NotFlour)?
                };
                c
            } else {
                false
            };

            // file is created after (de)serializing, see issue #1
            let mut out_file;

            match bxcad_type {
                BXCADType::BCCAD => {
                    let bccad = if indexized {
                        let bxcad_wrapper: BXCADWrapper<IndexizedBCCAD> =
                            serde_json::from_str(&json_)?;
                        bxcad_wrapper.indexized_to_bxcad()?
                    } else {
                        let bxcad_wrapper: BXCADWrapper<BCCAD> = serde_json::from_str(&json_)?;
                        bxcad_wrapper.to_bxcad()?
                    };
                    out_file = File::create(&bxcad)?;
                    bccad.to_binary(&mut out_file)?;
                }
                BXCADType::BRCAD => {
                    let brcad = if indexized {
                        let bxcad_wrapper: BXCADWrapper<IndexizedBRCAD> =
                            serde_json::from_str(&json_)?;
                        bxcad_wrapper.indexized_to_bxcad()?
                    } else {
                        let bxcad_wrapper: BXCADWrapper<BRCAD> = serde_json::from_str(&json_)?;
                        bxcad_wrapper.to_bxcad()?
                    };
                    out_file = File::create(&bxcad)?;
                    brcad.to_binary(&mut out_file)?;
                }
                //  BXCADType::Custom(_) => todo!(),
                _ => unimplemented!(),
            }
            println!(
                "Deserialized {:?} to {:?}",
                json.into_os_string(),
                bxcad.into_os_string()
            );
        }
    }
    Ok(())
}
