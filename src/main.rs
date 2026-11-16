use clap::{Parser, Subcommand};

use isoextract::{Error, Exit, Result};

use isoextract::dispatch::{ArgsDispatcher, ParserDispatcher, SubcommandDispatcher};

#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = "isoextract command-line")]
pub struct IsoextractCli {
    #[command(subcommand)]
    command: TopLevelCommand,
}
impl IsoextractCli {
    pub fn command(&self) -> TopLevelCommand {
        self.command.clone()
    }
}

impl ParserDispatcher<Error> for IsoextractCli {
    fn dispatch(&mut self) -> Result<()> {
        self.command.dispatch()?;

        Ok(())
    }
}

#[derive(Subcommand, Debug, Clone)]
pub enum TopLevelCommand {
    Hello(HelloOpt),
}
impl SubcommandDispatcher<Error> for TopLevelCommand {
    fn dispatch(&mut self) -> Result<()> {
        match self {
            TopLevelCommand::Hello(op) => op.dispatch()?,
        }
        Ok(())
    }
}

#[derive(Parser, Debug, Clone)]
pub struct HelloOpt {
    #[arg()]
    text: Vec<String>,
}
impl HelloOpt {
    pub fn text(&self) -> String {
        self.text.join(" ")
    }
}
impl ArgsDispatcher<Error> for HelloOpt {
    fn dispatch(&mut self) -> Result<()> {
        println!("{}", &self.text());

        Ok(())
    }
}

fn main() -> Exit {
    IsoextractCli::main()
}
