mod backend;

#[cfg(test)]
mod tests;

use backend::toplevel::Module;

use std::{
    error::Error,
    fmt::Display,
    fs::File,
    io::{BufReader, Read},
    path::{Path, PathBuf},
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

pub fn transpile_file<P: AsRef<Path>>(
    in_file: P,
    out_file: Option<P>,
) -> Result<(), Box<dyn Error>> {
    let infile = if let Some(ext) = in_file.as_ref().extension() {
        if ext != "json" {
            println!(
                "not transpiling {}: file extension doesn't end in .json, skipping.",
                in_file.as_ref().display()
            );
            return Ok(());
        }

        in_file.as_ref()
    } else {
        println!(
            "not transpiling {}: file doesnt have an extension, skipping.",
            in_file.as_ref().display()
        );
        return Ok(());
    };

    let infile_file = File::open(&infile)?;
    let infile_reader = BufReader::new(infile_file);

    let file: Module = match serde_json::de::from_reader(infile_reader) {
        Ok(data) => data,
        Err(err) => {
            return Err(Box::new(TranspileError::new(
                Box::new(err),
                format!("While parsing the JSON file at {}", infile.display()),
            )))
        }
    };

    file.write_file(out_file);

    Ok(())
}

pub fn batch_transpile<P: AsRef<Path>>(in_dir: P, out_dir: P) -> Result<(), Box<dyn Error>> {
    fn transpile_dir(in_dir: &Path, outdir: &Path) -> Result<(), Box<dyn Error>> {
        let needed_files_iter = in_dir
            .iter()
            .filter(|file_or_dir| !PathBuf::from(file_or_dir).is_dir())
            .map(|file| PathBuf::from(file));

        for f in needed_files_iter {
            let target_path = outdir.join(f.file_name().unwrap());
            transpile_file(&f, Some(&target_path))?
        }

        Ok(())
    }

    let needed_dirs_iter = in_dir
        .as_ref()
        .iter()
        .filter(|file_or_dir| PathBuf::from(file_or_dir).is_dir())
        .map(|dir| PathBuf::from(dir));

    for d in needed_dirs_iter {
        transpile_dir(&d, &out_dir.as_ref().join(&d.file_name().unwrap()))?
    }

    transpile_dir(Path::new("./"), out_dir.as_ref())?;

    Ok(())
}
