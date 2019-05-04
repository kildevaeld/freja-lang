


exports.expressions = [
    ["Assign", ["destination: Expr", "value: Expr", "operator: AssignmentOperator"]],
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
    ["Var", ["name: String", "initializer: ?Expr"]],
    ["VarList", ["variables: Vec<VarStmt>"]],
    ["Expr", ["expression: Expr"]],
    ["Func", ["name: String", "body: Stmt", "parameters: *Argument"]],
    ["Class", ["name: String", "members: *Stmt", "extends: ?String", "implements: *String"]],
    ["Interface", ["name: String", "extends: ?String", "members: *Stmt"]],
    ["Block", ["statements: *Stmt"]],
    ["If", ["test: Expr", "consequent: Stmt", "alternative: ?Stmt"]],
    ["For", ["element: Token", "index: ?Token", "iterator: Expr", "body: Stmt"]],
    ["Return", ["expression: ?Expr"]],
    ["Continue", []],
    ["Break", []]
];

exports.literals = [
    // ["Type", [
    //     ["Integer", "Double", "String", "Array", "Map", "Boolean"]
    // ]],
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
            "Eq", "Neq", "Lt", "Lte", "Gt", "Gte", "Is"
        ]
    ],
    ["ComparisonOperator",
        ["Eq", "Neq", "Lt", "Lte", "Gt", "Gte"]
    ],
    ["LogicalOperator",
        ["And", "Or"]
    ],
    ["AssignmentOperator",
        [
            "Add", "Sub", "Mul", "Div", "Mod", "ShiftLeft", "ShiftRight",
            "BitwiseAnd", "BitwiseOr", "BitwiseXor", "Assign"
        ]
    ],
    // ["Number", [
    //     ["Double", "f64"],
    //     ["Integer", "i64"]
    // ]],
    ["Literal", [
        ["String", "String"],
        ["Number", "Number"],
        ["Boolean", "bool"],
        ["Array", "Vec<Expr>"],
        ["Object", "Object"],
        "Null",
    ]],
    ["Argument", [
        ["Regular", "String"],
        ["Rest", "String"]
    ]]
];

