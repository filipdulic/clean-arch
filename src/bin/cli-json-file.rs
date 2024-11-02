use clap::Parser;
use clean_arch::infrastructure::interface::cli;
use clean_arch::infrastructure::utils::storage::data_storage;
use std::{path::PathBuf, sync::Arc};

#[derive(Parser)]
struct Args {
    #[clap(subcommand)]
    command: cli::Command,
    #[clap(help = "Directory to store data ", long)]
    data_dir: Option<PathBuf>,
}

pub fn main() {
    let args = Args::parse();
    let db = Arc::new(data_storage(args.data_dir));
    cli::run(db, args.command);
}
