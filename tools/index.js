const hbs = require('hbs'),
    fs = require('mz/fs');

function flatten(arr) {
    return arr.reduce(function (flat, toFlatten) {
        return flat.concat(Array.isArray(toFlatten) ? flatten(toFlatten) : toFlatten);
    }, []);
}

const expressions = [
    ["Assign", ["destination: Expr", "value: Expr"]],
    ["Call", ["member: Expr", "arguments: Expr"]],
    ["Literal", ["value: Literal"]],
    ["Binary", ["left: Expr", "right: Expr", "operator: Token"]],
    ["Member", ["object: Expr", "property: Expr", "computed: bool"]],
    ["Lookup", ["token: Token"]],
    ["Arguments", ["expressions: *Expr"]],
    ["Logical", ["left: Expr", "right: Expr", "operator: Token"]]
];

const statements = [
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

const literals = [
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


function generateStucture(list, base, ref = false) {
    return list.map(m => {
        return {
            name: m[0] + base,
            fields: m[1].map(inner => {
                let [name, value] = inner.split(':').map(m => m.trim())
                let isOptional = value[0] == '?';
                if (isOptional)
                    value = value.substr(1);
                let isMulti = value[0] == '*';
                if (isMulti)
                    value = value.substr(1);
                let isBox = value == base;
                value = (() => {
                    switch (value) {
                        case "Expr":
                        case "Stmt":
                        case "Literal":
                        case "Argument":
                        case "Token":
                            let token = value + "<'a>";
                            if (isBox) token = `Box<${token}>`;
                            if (isMulti) token = `Vec<${token}>`;
                            if (isOptional) token = `Option<${token}>`;
                            return token;
                        default:
                            return value;
                    }
                })();
                return {
                    name,
                    value
                }
            })
        }
    })
}


const data = (enums) => {

    return {
        enums: literals.map(m => {
            return {
                name: m[0],
                variants: m[1].map(field => {
                    return {
                        name: Array.isArray(field) ? field[0] : field,
                        value: Array.isArray(field) ? field[1] : void 0
                    }
                })
            }
        }).concat([{
            name: "Stmt",
            flatten: true,
            visitor: statements.map(m => {

                return m[0]
            }),
            variants: statements.map(m => {

                return {
                    name: m[0],
                    value: m[0] + "Stmt" + (m[1].length ? "<'a>" : '')
                }
            })
        }, {
            name: "Expr",
            flatten: true,
            visitor: expressions.map(m => m[0]),
            variants: expressions.map(m => {
                return {
                    name: m[0],
                    value: m[0] + "Expr" + (m[1].length ? "<'a>" : '')
                }
            })
        }]),

        structures: [
            ...generateStucture(statements, 'Stmt'),
            ...generateStucture(expressions, 'Expr')
        ]

    }
}


async function generate() {
    hbs.registerHelper('lower', (term) => {
        return term.toLowerCase()
    })
    let input = await fs.readFile(__dirname + '/template.hbs', 'utf8');
    let t = hbs.handlebars.compile(input);
    return t(data())
}

generate().then(out => {
    process.stdout.write(out);
})