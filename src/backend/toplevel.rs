use std::{
    fs::File,
    io::{BufWriter, Write},
    path::{Path, PathBuf},
};

use super::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub struct Module {
    name: String,
    content: Box<[Value]>,
}

#[typetag::serde]
impl Codegen for Module {
    fn code_gen(&self) -> String {
        self.content
            .iter()
            .map(|expr| expr.code_gen())
            .collect::<Vec<String>>()
            .join("\n")
    }
}

impl Module {
    pub fn write_file<P: AsRef<Path>>(&self, out: Option<P>) {
        let out = if let Some(of) = out {
            of.as_ref().to_path_buf()
        } else {
            let mut p = PathBuf::from(&self.name);
            p.set_extension("py");
            p
        };

        let file_name = if out.extension().is_none() {
            let mut out = out.clone();
            out.set_extension(".py");
            out
        } else {
            out.clone()
        };

        let res = self.code_gen();
        let file = File::create(&out).unwrap_or_else(|e| {
            panic!("failed to create a file at {}: {}", file_name.display(), e)
        });

        let mut file = BufWriter::new(file);
        file.write_all(res.as_bytes()).unwrap_or_else(|e| {
            panic!("failed to write the file at {}: {}", file_name.display(), e)
        });
    }
}
