use super::{
    stmt::{DefStmt, ForStmt, IfStmt},
    Block, Ident,
};
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
        indents: None,
        content: Box::new([
            Box::new(ident_3.clone()),
            Box::new(ident_3.clone()),
            Box::new(ident_3),
        ]),
    };

    let block2 = Block {
        indents: None,
        content: Box::new([
            Box::new(ident_2.clone()),
            Box::new(ident_2.clone()),
            Box::new(ident_2),
            Box::new(block3),
        ]),
    };

    let block1 = Block {
        indents: None,
        content: Box::new([
            Box::new(ident_1.clone()),
            Box::new(ident_1.clone()),
            Box::new(ident_1),
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

#[test]
fn test_forstmt() {
    let code = r#"
{
    "type": "ForStmt",
    "iter_vals": [{ "type": "Ident", "name": "i" }],
    "iter_subj": {
        "type": "CallStmt",
        "base": { "type": "Ident", "name": "range" },
        "args": [{ "type": "Literal", "value": "5" }],
        "kw_args": []
    },
    "content": {
        "type": "Block",
        "content": [
            {
                "type": "CallStmt",
                "base": { "type": "Ident", "name": "print"},
                "args": [{ "type": "Literal", "value": "f\"count={i}\"" }],
                "kw_args": []
            }
        ]
    }
    
}
    "#;

    let ast: ForStmt = serde_json::from_str(code).unwrap();
    let gen_code = ast.code_gen();
    println!("{}", gen_code)
}

#[test]
fn test_fib() {
    let code = r#"
{
    "type": "DefStmt",
    "name": { "type": "Ident", "name": "fibs" },
    "args": [{ "type": "Ident", "name": "n" }],
    "kw_args": [],
    "content": { "type": "Block", "content": [
        {
            "type": "DefStmt",
            "name": { "type": "Ident", "name": "fib" },
            "args": [{ "type": "Ident", "name": "n" }],
            "kw_args": [],
            "content": { "type": "Block", "content": [
                { "type": "IfStmt", "branches": [
                    {
                        "type": "If",
                        "condition": {
                            "type": "CmpExpr",
                            "op": "Leq",
                            "values": [
                                { "type": "Ident", "name": "n" },
                                { "type": "Literal", "value": "1" }]
                        },
                        "content": { "type": "Block", "content": [
                            { "type": "ReturnStmt", "value": { "type": "Ident", "name": "n" } }
                        ]}
                    }
                ]},
                {
                    "type": "ReturnStmt",
                    "value": {
                        "type": "MathExpr",
                        "op": "Add",
                        "values": [
                            {
                                "type": "CallStmt",
                                "base": { "type": "Ident", "name": "fib" },
                                "args": [
                                    { "type": "MathExpr", "op": "Sub", "values": [
                                        { "type": "Ident", "name": "n" },
                                        { "type": "Literal", "value": "1" }
                                    ]}
                                ],
                                "kw_args": []
                            },
                            {
                                "type": "CallStmt",
                                "base": { "type": "Ident", "name": "fib" },
                                "args": [
                                    { "type": "MathExpr", "op": "Sub", "values": [
                                        { "type": "Ident", "name": "n" },
                                        { "type": "Literal", "value": "2" }
                                    ]}
                                ],
                                "kw_args": []
                            }
                        ]
                    }
                }
            ]}
        },
        {
            "type": "ForStmt",
            "iter_vals": [
                { "type": "Ident", "name": "i" }
            ],
            "iter_subj": {
                "type": "CallStmt",
                "base": { "type": "Ident", "name": "range" },
                "args": [{ "type": "Ident", "name": "n" }],
                "kw_args": []
            },
            "content": { "type": "Block", "content": [
                {
                    "type": "YieldStmt",
                    "value": {
                        "type": "CallStmt",
                        "base": { "type": "Ident", "name": "fib" },
                        "args": [
                            {
                                "type": "MathExpr",
                                "op": "Add",
                                "values": [
                                    { "type": "Ident", "name": "i" },
                                    { "type": "Literal", "value": "1" }
                                ]
                            }
                        ],
                        "kw_args": []
                    }
                }   
            ]}
        }
    ]}
}
    "#;

    let ast: DefStmt = serde_json::from_str(code).unwrap();
    let gen_code = ast.code_gen();
    println!("{}", gen_code)
}
