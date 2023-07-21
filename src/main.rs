use std::{fs, path::PathBuf};

use clap::{Parser, Subcommand};
use cursython::{batch_transpile, transpile_file};

#[derive(Subcommand)]
pub enum Command {
    Compile {
        #[arg(short = 'o')]
        outdir: Option<PathBuf>,
        infile: PathBuf,
    },
    BatchCompile {
        outdir: PathBuf,
        indir: PathBuf,
    },
}

#[derive(Parser)]
#[command(author, version, about, long_about=None)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

fn main() {
    color_eyre::install().unwrap();

    match Args::parse().command {
        Command::Compile { outdir, infile } => {
            if !infile.exists() {
                panic!("the file at {} doesn't exist.", infile.display())
            }

            transpile_file(infile, outdir.unwrap_or(PathBuf::from("./"))).unwrap();
        }
        Command::BatchCompile { outdir, indir } => {
            if !outdir.exists() {
                fs::create_dir(&outdir).unwrap();
            };

            batch_transpile(indir, outdir).unwrap();
        }
    };
}
