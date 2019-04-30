const hbs = require('hbs'),
    fs = require('mz/fs'),
    {
        expressions,
        statements,
        literals
    } = require('./ast');

function flatten(arr) {
    return arr.reduce(function (flat, toFlatten) {
        return flat.concat(Array.isArray(toFlatten) ? flatten(toFlatten) : toFlatten);
    }, []);
}

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
                            // let token = value + "<'a>";
                            let token = value;
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


const data = () => {
    return {
        enums: literals.map(m => {

            return {
                name: m[0],
                variants: m[1].map(field => {
                    if (Array.isArray(field)) {
                        return {
                            name: Array.isArray(field) ? field[0] : field,
                            value: Array.isArray(field) ? field[1] : void 0,

                        }
                    } else {
                        return { name: field }
                    }

                }),
                lifetime: m[2]
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
                    // value: m[0] + "Stmt" + (m[1].length ? "<'a>" : '')
                    value: m[0] + "Stmt"
                }
            }),
            lifetime: true
        }, {
            name: "Expr",
            flatten: true,
            visitor: expressions.map(m => m[0]),
            variants: expressions.map(m => {
                return {
                    name: m[0],
                    // value: m[0] + "Expr" + (m[1].length ? "<'a>" : '')
                    value: m[0] + "Expr"
                }
            }),
            lifetime: true
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