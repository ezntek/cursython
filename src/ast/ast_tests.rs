use std::{
    fs::File,
    io::{BufWriter, Write},
};

use super::{Block, Ident};
use crate::codegen::Codegen;

#[test]
fn test_nested_blocks() {
    let ident_1 = Ident {
        name: "foo".to_owned(),
    };

    let ident_2 = Ident {
        name: "bar".to_owned(),
    };

    let ident_3 = Ident {
        name: "baz".to_owned(),
    };

    let block3 = Block {
        indented_lvls: None,
        content: Box::new([
            Box::new(ident_3.clone()),
            Box::new(ident_3.clone()),
            Box::new(ident_3.clone()),
        ]),
    };

    let block2 = Block {
        indented_lvls: None,
        content: Box::new([
            Box::new(ident_2.clone()),
            Box::new(ident_2.clone()),
            Box::new(ident_2.clone()),
            Box::new(block3),
        ]),
    };

    let block1 = Block {
        indented_lvls: None,
        content: Box::new([
            Box::new(ident_1.clone()),
            Box::new(ident_1.clone()),
            Box::new(ident_1.clone()),
            Box::new(block2),
        ]),
    };

    let code = block1.code_gen();

    let f = File::create("out.txt").unwrap();
    let mut f = BufWriter::new(f);

    f.write_all(code.as_bytes()).unwrap();
}
