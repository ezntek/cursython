use std::{
    fs::File,
    io::{BufWriter, Write},
};

use serde::Serialize;

use super::{stmt::IfStmt, Block, Ident};
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

    println!("{}", code)
}

#[test]
fn test_deser() {
    let ident = Ident {
        name: "foo".to_owned(),
    };

    let deser_data = serde_json::to_string(&ident).unwrap();
    println!("{}", deser_data);
}

#[test]
fn test_ifstmt() {
    let code = r#"
{
    "type": "IfStmt",
    "branches": [
        {
            "type": "If",
            "condition": {
                "type": "CmpExpr",
                "op": "Gt",
                "values": [{"type": "Literal", "value": "1"},
                           {"type": "Literal", "value": "2"}]
            },
            "content": {
                "type": "Block",
                "content": [
                    {
                        "type": "CallStmt",
                        "base": {
                            "type": "Ident",
                            "name": "print"
                        },
                        "args": [
                            {"type": "Literal", "value": "\"maths is broken!\""}
                        ],
                        "kw_args": []
                    }
                ]
            }
        }
    ]
}
    "#;

    let ast: IfStmt = serde_json::from_str(code).unwrap();
    let gen_code = ast.code_gen();
    println!("{}", gen_code)
}
