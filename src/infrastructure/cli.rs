use super::storage::data_storage;
use crate::cli::Command;
use clap::Parser;
use std::{path::PathBuf, sync::Arc};

#[derive(Parser)]
struct Args {
    #[clap(subcommand)]
    command: Command,
    #[clap(help = "Directory to store data ", long)]
    data_dir: Option<PathBuf>,
}

pub fn run() {
    let args = Args::parse();
    let db = Arc::new(data_storage(args.data_dir));
    crate::cli::run(db, args.command);
}
