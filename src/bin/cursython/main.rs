use std::{fs, path::PathBuf};

use clap::{Parser, Subcommand};
use cursython::{batch_transpile, transpile_file};

#[derive(Subcommand)]
pub enum Command {
    Compile {
        #[arg(short = 'o')]
        outfile: Option<PathBuf>,
        infile: PathBuf,
    },
    BatchCompile {
        outdir: PathBuf,
        indir: PathBuf,
    },
}

#[derive(Parser)]
#[command(author, version, long_about=None)]
#[command(about = "The Curysthon Compiler/Transpiler.")]
struct Args {
    #[command(subcommand)]
    command: Command,
}

fn main() {
    color_eyre::install().unwrap();

    match Args::parse().command {
        Command::Compile { outfile, infile } => {
            if !infile.exists() {
                panic!("the file at {} doesn't exist.", infile.display())
            }

            transpile_file(infile, outfile).unwrap();
        }
        Command::BatchCompile { outdir, indir } => {
            if !outdir.exists() {
                fs::create_dir(&outdir).unwrap();
            };

            batch_transpile(indir, outdir).unwrap();
        }
    };
}
