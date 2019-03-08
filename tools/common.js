exports.expressions = [
    ["Assign", ["destination: Expr", "value: Expr"]],
    ["Call", ["member: Expr", "arguments: Expr"]],
    ["Literal", ["value: Literal"]],
    ["Binary", ["left: Expr", "right: Expr", "operator: Token"]],
    ["Member", ["object: Expr", "property: Expr", "computed: bool"]],
    ["Lookup", ["token: Token"]],
    ["Arguments", ["expressions: *Expr"]],
    ["Logical", ["left: Expr", "right: Expr", "operator: Token"]]
];

exports.statements = [
    ["Program", ["statements: *Stmt"]],
    ["Var", ["name: Token", "initializer: ?Expr"]],
    ["VarList", ["variables: Vec<VarStmt<'a>>"]],
    ["Expr", ["expression: Expr"]],
    ["Func", ["name: Token", "body: Stmt", "parameters: *Argument"]],
    ["Class", ["name: Token", "members: *Stmt"]],
    ["Block", ["statements: *Stmt"]],
    ["If", ["test: Expr", "consequent: Stmt", "alternative: ?Stmt"]],
    ["For", ["element: Token", "index: ?Token", "iterator: Expr", "body: Stmt"]],
    ["Return", ["expression: ?Expr"]],
    ["Continue", []],
    ["Break", []]
];

exports.literals = [
    ["Literal", [
        ["String", "&'a str"],
        ["Number", "Number<'a>"],
        ["Boolean", "bool"],
    ]],
    ["Number", [
        ["Double", "&'a str"],
        ["Integer", "&'a str"]
    ]],
    ["Argument", [
        ["Regular", "&'a str"],
        ["Rest", "&'a str"]
    ]],
];