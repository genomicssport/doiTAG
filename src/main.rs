mod args;
use crate::args::CommandParse;
use crate::args::Commands;
use clap::Parser;
mod doigen;
use crate::doigen::generatetag;

/*
 Author Gaurav Sablok
 Instytut Chemii Bioorganicznej
 Polskiej Akademii Nauk
 ul. Noskowskiego 12/14 | 61-704, Poznań
 Date: 2025-22-8
*/

#[tokio::main]
async fn main() {
    let argparse = CommandParse::parse();
    match &argparse.command {
        Commands::Doigen { pathfile } => {
            let command = generatetag(pathfile).unwrap();
            println!("The command has finished:{}", command);
        }
    }
}
