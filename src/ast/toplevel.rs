use std::{
    fs::File,
    io::{BufWriter, Write},
    path::Path,
};

use super::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub struct Module {
    name: String,
    content: Box<[Value]>,
    code_memo: Option<String>,
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
    pub fn write<P: AsRef<Path>>(&self, path: P) {
        let res = self.code_gen();
        let file = File::create(path.as_ref()).unwrap_or_else(|e| {
            panic!(
                "failed to create a file at {}: {}",
                path.as_ref().display(),
                e
            )
        });

        let mut file = BufWriter::new(file);
        file.write_all(res.as_bytes()).unwrap_or_else(|e| {
            panic!(
                "failed to write the file at {}: {}",
                path.as_ref().display(),
                e
            )
        });
    }
}
