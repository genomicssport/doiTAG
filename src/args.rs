use clap::{Parser, Subcommand};
#[derive(Debug, Parser)]
#[command(
    name = "doigen",
    version = "1.0",
    about = "doigen.
       ************************************************
      Gaurav Sablok, IBCH, PAN, Poznan, Poland,
      https://portal.ichb.pl/laboratory-of-genomics/.
      Email: gsablok@ibch.poznan.pl
      ************************************************"
)]
pub struct CommandParse {
    /// subcommands for the specific actions
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// generate doi for each sequences
    Doigen {
        /// provide ONT file
        pathfile: String,
    },
}
