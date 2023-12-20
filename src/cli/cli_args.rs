

use clap::Args;
use std::path::PathBuf;

#[derive(Args)]
pub struct UpArgs {
    pub path: PathBuf
}

#[derive(Args)]
pub struct DownArgs {
    code: String
}

