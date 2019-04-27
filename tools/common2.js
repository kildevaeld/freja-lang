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
    ["VarList", ["variables: Vec<VarStmt>"]],
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
        ["String", "String"],
        ["Number", "Number"],
        ["Boolean", "bool"],
    ], false],
    ["Number", [
        ["Double", "f64"],
        ["Integer", "i64"]
    ], false],
    ["Argument", [
        ["Regular", "String"],
        ["Rest", "String"]
    ], true],
];