

mod models {
    pub mod cli_parse;
}

mod communication {
    pub mod connect;
}

use crate::models::cli_parse::{ BeamCli, LaserbeamCommands };

use clap::Parser;

fn main() {
    let value = BeamCli::parse();
    
    match &value.cmd {
        LaserbeamCommands::Up(values) => {
            // Initiate up
        },
        LaserbeamCommands::Down(values) => {
            // Initiate down
        },
        LaserbeamCommands::Config => {
            // Initiate configuration
        }
    }
}
