pub mod cli {
    pub mod cli_parse;
    pub mod cli_args;
}

pub mod communication {
    pub mod communication;
}

pub mod misc;
pub mod types;

use communication::communication::establish_beam;

use cli::cli_parse::{ BeamCli, LaserbeamCommands };

use clap::Parser;

fn main() {
    let value = BeamCli::parse();
    
    match &value.cmd {
        LaserbeamCommands::Up(values) => {
            // Initiate up
            let session = establish_beam("http://localhost:8080", Box::new(values.path.to_owned()))
            .expect("Beam establishment failed");

            println!("{:?}", session);

        },
        LaserbeamCommands::Down(values) => {
            // Initiate down
        },
        LaserbeamCommands::Config => {
            // Initiate configuration
        }
    }
}
