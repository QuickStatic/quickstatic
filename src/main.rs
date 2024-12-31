mod commands;
mod markdown;
mod html_generator;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Build {
        #[clap(short, long)]
        folder: Option<String>,
    },
    Serve,
}

fn main() {
    let args = Args::parse();

    match &args.command {
        Some(Commands::Build { folder }) => {
            let folder = folder.clone().unwrap_or("src".parse().unwrap());
            commands::build(folder)
        },
        _ => {}
    }
}
