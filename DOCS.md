# How to write Python code with JSON™

NOTE: the docs are pure shit, I wrote this while braindead. I really want to install openbsd rn. send help pls

## Prereqs

This is an internally tagged struct.

```json
{
    "type": "TheStructType"
    ...
}
```

This will be a recurring theme throughout this long long long long long guide. This indicates the type of the struct, so the JSON parser wont choke.

This is JSON5.

```json5
{
    type: "TheStructType",
}
```

Its basically JSON but a lot less painful to write, because you can put trailing commas and keys dont have to have double quotes, among other features.

Almost everything below (the identifiers, literals, etc.) can be replaced with litreally any other code element. you can SetStmt a tuple, you can CallStmt a PropertyStmt. please do read the code in the [backend folder](/src/backend) for more info. the structures in `stmt.rs`, `ops.rs` and `stmt.rs` should be readable.

Anything that supports every expression is called a `Value` in the code, or a heap-allocated trait object that can generate code, or `Box<dyn Codegen>`. If you see a `Value` that means you can put anything there. even function definitions and if statements (the code wont run, though.)

## Note

I will probably write JSON5 exclusively from now on because JSON is PAIN.
JSON5 is very similar to JSON anyway, you should be able to read it without any hiccups.

## Identifiers, Literals and Expressions


This is an identifier. An identifier is like a variable name, keyword argument name, function definition argument name, class name, module name, or any other name in the code that identifies something.
```json5
{
    type: "Ident",
    name: "foo"
}
```

This is a literal. Literals represents a value, like a string or an integer. The code generation logic for this is the same for the identifier, but please use this or else its cheating.

```json5
{
    type: "Literal",
    value: "\"bar\""
}
```

Here, numbers are strings. strings are strings with escaped double quotes inside. booleans are strings (`"True"` or `"False"`). Everything is case sensitive, too. Have fun!

This is a mathematical expression (an addition expression). It compiles to this: `(1 + 2 + 3)`, with the operation placed between every element inside `values`.

```json5
{
    type: "MathExpr",
    op: "Add", // can be "Add", "Sub" (tract), "Mul" (tiply), "Div" (ide), "Pow" (er).
    values: ["1", "2", "3"],
}
```

This is a comparison expression. It compiles to `(1 > 2)`, with the operation placed between each element inside `values`.

```json5
{
    type: "CmpExpr",
    op: "Gt", // can be Gt, Lt, Eq, Geq, Leq, Neq. (Greater than, lesse than, equal to, greater than or equal to, less than or equal to, and not equal to respectively.)
    values: ["1", "2"]
}
```


The syntax is the same for logical and bitwise expressions. Read [the code](/src/backend/ops.rs) for more info.

## some more complicated expressinos

this is a tuple expression. it compiles to `(1, 2)`.

```json5
{
    type: "TupleExpr",
    elems: [{ type: "Literal", value: "1"}, { type: "Literal", value: "2"}]
}
```


This is a list expression. the syntax is the same as the tuple expression, except this compiles to `[6, 7, 8]`.

```json5
{
    type: "ListExpr",
    elems: [
        { type: "Literal", value: "6" },
        { type: "Literal", value: "7" },
        { type: "Literal", value: "8" }
    ]
}
```

This is a dictionary expression. key-value pairs are represented with the `DictKvPair` struct, and the syntax is shown above.

```json5
{
    type: "DictExpr",
    elems: [
        { type: "DictKvPair", key: { type: "Literal", value: "\"foo\"" }, key: { type: "Literal", value: "\"bar\"" } },
        { type: "DictKvPair", key: { type: "Literal", value: "\"baz\"" }, key: { type: "Literal", value: "\"quux\"" } }
    ]
}
```


try saving this to a file and compiling it.

## The block

```py
:
    # code
```

This is a python block. A colon, a newline and an indent, followed by a line of code.

The JSON representation for this would be:

```json5
{
    type: "Block",
    content: [
        // code here
    ]
}
```

This will be a recurring theme from here. Every single statement involving a block, such as the if statement, while statement, for statement, function (def) statement and a few others will use the block object.

## Statements
Statements arent expressions.

This is a set statement. it sets one value to another. Tuple assignments are supported with the tuple statement.
```json5
{
    type: "SetStmt",
    name: { type: "Ident", name: "foo" },
    value: { type: "Literal", value: "69" },
}
```


This is an import statement. It compiles to `import sys`.

```json5
{
    type: "ImportStmt",
    mod_name: { type: "Ident", name: "sys" },
}
```

This is a from-import statement. It compiles to `from sys import exit`.

```json5
{
    type: "FromImportStmt",
    import_from: { type: "Ident", name: "sys" },
    mod_name: { type: "Ident", name: "exit" },
}
```

This is an import-as statement. it compiles to `import sys as not_sys`.

```json5
{
    type: "ImportAsStmt",
    mod_name: { type: "Ident", name: "sys" },
    import_as: { type: "Ident", name: "not_sys" },
}
```

This is a call statement.

```json5
{
    type: "CallStmt",
    base: { type: "Ident", name: "print" },
    args: [{ type: "Ident", name: "foo" }],
    kw_args: [{ type: "Kwarg", name: { type: "Ident", name: "end"}, value: { type: "Literal", value: "\" \"" }],
}
```

This code compiles to `print(foo, end=" ")`. the base is the function name, the arguments are the positional arguments and the keyword arguments are the keyword arguments.

This is a return statement. it compiles to `return 5`.

```json5
{
    type: "ReturnStmt",
    value: { type: "Literal", value: "5" }
}
```

This is a yield statement. it compiles to `yield foo`.

```json5
{
    type: "YieldStmt",
    value: { type: "Ident", value: "foo" }
}
```

Now you can write lazily evaluated iterators in JSON I mean Python!

This is a property Statement.

```json5
{
    type: "PropertyStmt",
    base: { type: "Ident", name: "sys" },
    props: [{ type: "Ident", name: "exit" }],
}
```

You can access a property on an identifier with this. every single element inside props will be seperated by a `.` when compiled. This code, for instance, compiles to `sys.exit`. You can even call it with a CallStmt:

```json5
{
    type: "CallStmt",
    base: {
        type: "PropertyStmt",
        base: { type: "Ident", name: "sys" },
        props: [{ type: "Ident", name: "exit" }],
    },
    args: [{ type: "Literal", value: "1" }],
    kw_args: [],
}
```

## More complicated statements (the ones with blocks)

The if statement consists of a list of branches.

```py
if name == "joe":
    ...
elif name == "jim":
    ...
else:
    ...
```

Each of those cases is a branch in this case, and the if statement is represented as the following:

```json5
{
    type: "IfStmt",
    branches: [
        {
            type: "If",
            condition: {
                type: "CmpExpr",
                op: "Eq",
                values: [
                    { type: "Ident", name: "name" },
                    { type: "Literal", name: "\"joe\"" },
                ],
            }, // most commonly a LogExpr or a CmpExpr
            content: [
                { type: "Ident", name: "Ellipsis" },
            ]
        },
        {
            type: "Elif",
            condition: {
                type: "CmpExpr",
                op: "Eq",
                values: [
                    { type: "Ident", name: "name" },
                    { type: "Literal", name: "\"jim\"" },
                ],
            }, // most commonly a LogExpr or a CmpExpr
            content: [
                { type: "Ident", name: "Ellipsis" },
            ]
        },
        {
            type: "Else",
            content: [
                { type: "Ident", name: "Ellipsis" },
            ]
        }
    ],
}
```

This is a for loop.

Oh, wait, you can just look at [this example.](/examples/basic_for_loop.py.json). Here's the python version:

```py
for i in range(5):
    print(f"count={i}")
```

The `iter_subj` is the latter part of the for loop, it is the value to be iterated over. In this case it is the range. `iter_vals` is the value to get out of the iterator, in this case it is the `i`. 

This is a while loop.

```json5
{
    type: "WhileStmt",
    cond: {
        // put your condition here, usually a LogExpr or a CmpExpr.
    },
    content: {
        type: "Block",
        content: [
            // loop body here.
        ]
    }
}
```

This is a function definition.

```json5
{
    type: "DefStmt",
    name: { type: "Ident", name: "Foo" },
    decorator: {}, // you can put in a decorator (OPTIONAL).
    args: [], // function arguments go here.
    kw_args: [], // function arguments with default values go here.
    content: {
        type: "Block",
        content: [], // code goes here.
    }
}
```

Lastly, the class definition. check out [these](/examples/cherrypy_webapp.py.json) [examples](/examples/pygobject_app.py.json) because I really dont want to write anymore docs.
