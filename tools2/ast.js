


exports.expressions = [
    ["Assign", ["destination: Expr", "value: Expr"]],
    ["Call", ["member: Expr", "arguments: Vec<Expr>"]],
    ["Literal", ["value: Literal"]],
    ["Binary", ["left: Expr", "right: Expr", "operator: BinaryOperator"]],
    ["Member", ["object: Expr", "property: Expr", "computed: bool"]],
    ["Lookup", ["token: Token"]],
    ["Arguments", ["expressions: *Expr"]],
    ["Logical", ["left: Expr", "right: Expr", "operator: LogicalOperator"]],
    ["This", []],
    ["Var", ["name: Token"]],
    ["Identifier", ["value: String"]],
    ["Unary", ["value: Expr", "operator: UnaryOperator"]],
    ["Postfix", ["value: Expr", "operator: PostfixOperator"]]
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
    ["UnaryOperator",
        ["Plus", "Minus", "Increment", "Decrement"]
    ],
    ["PostfixOperator",
        ["Increment", "Decrement"]
    ],
    ["BinaryOperator",
        [
            "Add", "Sub", "Mul", "Div", "Mod",
            "BitwiseXor", "BitwiseAnd", "BitwiseOr",
            "ShiftLeft", "ShiftRight",
            "Eq", "Neq", "Lt", "Lte", "Gt", "Gte"
        ]
    ],
    ["ComparisonOperator",
        ["Eq", "Neq", "Lt", "Lte", "Gt", "Gte"]
    ],
    ["LogicalOperator",
        ["And", "Or"]
    ],
    ["Number", [
        ["Double", "f64"],
        ["Integer", "i64"]
    ]],
    ["Literal", [
        ["String", "String"],
        ["Number", "Number"],
        ["Boolean", "bool"],
        "Null",
    ]],
    ["Argument", [
        ["Regular", "String"],
        ["Rest", "String"]
    ]],
];

