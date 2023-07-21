mod ast;
mod codegen;

use crate::ast::toplevel::Module;
use std::{
    error::Error,
    fmt::Display,
    fs::File,
    io::{self, BufReader},
    path::PathBuf,
};

#[derive(Debug)]
pub struct TranspileError {
    cause: Box<dyn Error>,
    desc: String,
}

impl TranspileError {
    fn new(cause: Box<dyn Error>, desc: String) -> Self {
        TranspileError { cause, desc }
    }
}

impl Error for TranspileError {
    fn cause(&self) -> Option<&dyn Error> {
        Some(&*self.cause)
    }

    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }

    fn description(&self) -> &str {
        self.desc.as_str()
    }
}

impl Display for TranspileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}: {}", self.desc, self.cause))
    }
}

pub fn transpile_file<P: ToOwned<Owned = PathBuf>>(
    infile: P,
    outdir: P,
) -> Result<(), Box<dyn Error>> {
    let infile = infile.to_owned();
    let outdir = if !outdir.to_owned().exists() {
        let err = io::Error::new(
            io::ErrorKind::NotFound,
            format!("the path at {} does not exist", outdir.to_owned().display()),
        );
        return Err(Box::new(err));
    } else {
        outdir.to_owned()
    };

    let infile_file = File::open(&infile)?;
    let mut infile_reader = BufReader::new(infile_file);

    let file: Module = match serde_json::from_reader(&mut infile_reader) {
        Ok(data) => data,
        Err(err) => {
            return Err(Box::new(TranspileError::new(
                Box::new(err),
                format!("While parsing the JSON file at {}", infile.display()),
            )))
        }
    };

    file.write_file(Some(&outdir));

    Ok(())
}
