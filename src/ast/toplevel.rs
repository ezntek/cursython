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
    pub fn write_file<P: AsRef<Path>>(&self, dir: Option<P>) {
        let dir = if let Some(d) = dir {
            d.as_ref().to_path_buf()
        } else {
            PathBuf::from("./")
        };

        let path = dir.join(format!("{}.py", self.name));

        let res = self.code_gen();
        let file = File::create(&path)
            .unwrap_or_else(|e| panic!("failed to create a file at {}: {}", path.display(), e));

        let mut file = BufWriter::new(file);
        file.write_all(res.as_bytes())
            .unwrap_or_else(|e| panic!("failed to write the file at {}: {}", path.display(), e));
    }
}
