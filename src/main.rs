use std::fs::File;

use clap::{Parser, Subcommand};
use iocore::Path;
use isoextract::{
    Error,
    Exit,
    Result,
    dispatch::{ArgsDispatcher, ParserDispatcher, SubcommandDispatcher},
};
use isomage::{detect_and_parse_filesystem, extract_node};

#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = "isoextract command-line")]
pub struct Cli {
    iso_file: Path,
    output_dir: Option<Path>,
}
impl Cli {
    pub fn file(&self) -> Result<Path> {
        if self.iso_file.extension() == Some(".iso".to_string()) {
            Ok(self.iso_file.clone())
        } else {
            Err(Error::RuntimeError(format!("not a iso file: {}", &self.iso_file)))
        }
    }
}

impl ParserDispatcher<Error> for Cli {
    fn dispatch(&mut self) -> Result<()> {
        let iso_path = self.file()?;
        let iso_str = iso_path.to_string();
        let output_dir = self.output_dir.clone().unwrap_or_else(|| iso_path.parent().unwrap());
        let output_str = output_dir.to_string();

        // Open the ISO image and automatically detect ISO 9660 or UDF
        let mut file = File::open(iso_path)?;
        let root = detect_and_parse_filesystem(&mut file, iso_str.as_str())?;

        // Extract the entire root tree safely into the target directory
        extract_node(&mut file, &root, output_str.as_str())?;

        Ok(())
    }
}

fn main() -> Exit {
    Cli::main()
}
