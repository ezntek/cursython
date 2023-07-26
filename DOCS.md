# How to write Python code with JSON™

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

## Note

I will probably write JSON5 exclusively from now on because JSON is PAIN.
JSON5 is very similar to JSON anyway, you should be able to read it without any hiccups.

## Identifiers, Literals and Expressions

```json5
{
    type: "Ident",
    name: "foo"
}
```

This is an identifier. An identifier is like a variable name, keyword argument name, function definition argument name, class name, module name, or any other name in the code that identifies something.

```json5
{
    type: "Literal",
    value: "\"bar\""
}
```

This is a literal. Literals represents a value, like a string or an integer. The code generation logic for this is the same for the identifier, but please use this or else its cheating.

Here, numbers are strings. strings are strings with escaped double quotes inside. booleans are strings (`"True"` or `"False"`). Everything is case sensitive, too.


