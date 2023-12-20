use clap::{Parser, Subcommand, command} ;

use self::cli_args::{UpArgs, DownArgs};

#[path ="cli_args.rs"]
mod cli_args;

// Defines the command args for each valid laser beam top-level command
#[derive(Subcommand)]
pub enum LaserbeamCommands {
    Up (UpArgs),
    Down (DownArgs),
    Config
}

// Parser for CL arguments
#[derive(Parser)]
#[command(author="Jordan Zdimirovic", version="1.0", about="Initiate a laserbeam session")]
pub struct BeamCli {
    #[command(subcommand)]
    pub cmd: LaserbeamCommands
}