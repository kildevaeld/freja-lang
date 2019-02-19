const expressions = [
    ["Assign", ["token: Token", "value: Expr"]],
    ["Call", ["callee: Expr", "parent: ?Expr", "arguments: *Expr"]],
    ["Literal", ["value: Literal"]],
    ["Binary", ["left: Expr", "right: Expr", "operator: Token"]]
];

const statements = [
    ["Var", ["name: Token", "initializer: Expr"]],
    ["Expr", ["expression: Expr"]],
    ["Func", ["name: Token", "body: Stmt", "parameters: *Expr"]],
    ["Block", ["statements: *Stmt"]],
    ["Return", ["expressions: ?Expr"]]
];

const literals = [
    ["String", ["String", "&'a str"]],
    ["Number", ["Number", "Number<'a>"]],
    ["Boolean", ["bool", "bool"]],
];

const number = [
    ["Double", ["f64", "&'a str"]],
    ["Integer", ["i64", "&'a str"]]
]

function generateEnums(name, fields, ref = false) {
    return [
        '#[derive(Serialize, Debug, Clone, PartialEq, Deserialize)]',
        '#[serde(tag = "type", content = "value")]',
        `pub enum ${name}${ref ? "Ref<'a>" : ''} {`,
        fields.map(m => {
            return `  ${m[0]}(${ref ? m[1][1] : m[1][0]})`
        }).join(',\n'),
        '}'
    ].join('\n');
}

console.log([
    generateEnums("Literal", literals, false),
    generateEnums("Number", number, false),
    generateEnums("Literal", literals, true),
    generateEnums("Number", number, true)
].join('\n'));


const prelude = `
use super::owned::*;
use super::traits::{ExprRefVisitor, StmtRefVisitor};
use std::fmt;

#[derive(Serialize, Debug, Clone, PartialEq, Eq, Hash, Deserialize)]
pub struct Location(pub usize, pub usize);

#[derive(Serialize, Debug, PartialEq, Eq, Hash, Clone)]
pub struct TokenRef<'a> {
    pub location: Location,
    pub value: &'a str,
}

impl<'a> TokenRef<'a> {
    pub fn new(location: Location, value: &'a str) -> TokenRef<'a> {
        TokenRef { location, value }
    }
}

impl<'a> fmt::Display for TokenRef<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

#[derive(Serialize, Debug, Clone, PartialEq, Deserialize)]
#[serde(tag = "type", content = "value")]
pub enum LiteralRef<'a> {
    String(&'a str),
    Number(NumberRef<'a>),
}

impl<'a> fmt::Display for LiteralRef<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LiteralRef::String(s) => write!(f, "{}", s),
            LiteralRef::Number(n) => <fmt::Display>::fmt(n, f),
        }
    }
}

#[derive(Serialize, Debug, Clone, PartialEq, Deserialize)]
#[serde(tag = "type", content = "value")]
pub enum NumberRef<'a> {
    Integer(&'a str),
    Float(&'a str),
}

impl<'a> fmt::Display for NumberRef<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match *self {
            NumberRef::Integer(s) => s,
            NumberRef::Float(n) => n,
        };
        write!(f, "{}", s)
    }
}
`;



function genStatementRef(exp, base, cmp, ref) {
    let output = [
        '#[derive(Serialize, Debug, Clone, PartialEq)]',
        `pub struct ${exp[0]}${base}${ref ? "Ref<'a>" : ''} {`,
        [
            //'  location: Location',
            '  location: Location',
            ...exp[1].map(m => {
                let tokens = m.split(':').map(m => m.trim());
                tokens[1] = (() => {
                    let isOptional = tokens[1][0] == '?';
                    if (isOptional)
                        tokens[1] = tokens[1].substr(1);

                    let isMulti = tokens[1][0] == '*';
                    if (isMulti)
                        tokens[1] = tokens[1].substr(1);

                    let isBox = tokens[1] == cmp;

                    switch (tokens[1]) {
                        case "Expr":
                        case "Stmt":
                        case "Literal":
                        case "Token":
                            let token = tokens[1] + (ref ? "Ref<'a>" : '');
                            if (isBox) token = `Box<${token}>`;
                            if (isMulti) token = `Vec<${token}>`;
                            if (isOptional) token = `Option<${token}>`;
                            return token;
                        default:
                            return tokens[1];
                    }
                })();
                return "  " + tokens.join(': ');
            }),
        ].filter(m => m != null).join(',\n'),
        '}'
    ];
    return output.join('\n');
}


function generateStatementRefs(list, base, visitor, hasLocation, cmp, ref) {
    return [
        '#[derive(Serialize, Debug, Clone, PartialEq)]',
        `pub enum ${base} {`,
        ...list.map(m => {
            return `  ${m[0]}(${m[0]}${base})`
        }),
        "}",
        `impl<'a> StmtRef<'a> {`,
        `  pub fn accept<R>(&self, visitor: &mut ${visitor}Visitor<R>) -> R {`,
        '    match self {',
        list.map(m => {
            return `      StmtRef::${m[0]}(e) => visitor.visit${m[0]}Stmt(&e)`
        }).join(',\n'),
        '    }',
        '  }',
        '}',
        ...list.map(expr => genStatementRef(expr, base, hasLocation, cmp, ref))
    ].join('\n')
}

function genAst() {
    return [
        prelude,
        generateStatementRefs(expressions, "Expr", "Expr", true),
        generateStatementRefs(statements, "Stmt", "Stmt", true),
        generateStatementRefs(expressions, "Expr", "Expr", false),
        generateStatementRefs(statements, "Stmt", "Stmt", false),

    ].join('\n');
}

function generateParserFunctions(list, base, postfix) {
    return list.map(m => {
        return [`fn parse_${m[0].toLowerCase()}_${postfix}<'a>(pair: Pair<'a, Rule>) -> ${base}<'a> {`,
            `  unimplemented!("parse_${m[0].toLowerCase()}: {:?}", pair.as_rule());`,
            `}`
        ].join('\n');
    }).join("\n");
}


const e = [
    genAst(),
    //generateParserFunctions(expressions, "ExprRef", "expr"),
    //generateParserFunctions(statements, "StmtRef", "stmt")
]

console.log(e.join('\n'));