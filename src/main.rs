use std::path::PathBuf;

use clap::Parser;
use cursython::*;

#[derive(Parser)]
#[command(author, version, about, long_about=None)]
struct Args {
    #[arg(short = 'o')]
    outdir: Option<PathBuf>,
    infile: PathBuf,
}

fn main() {
    color_eyre::install().unwrap();

    let args = Args::parse();
    transpile_file(args.infile, args.outdir.unwrap_or(PathBuf::from("./"))).unwrap();
}
