use std::path::PathBuf;

use clap::Args;

#[derive(Args)]
pub struct UpArgs {
    pub path: PathBuf
}

#[derive(Args)]
pub struct DownArgs {
    code: String
}