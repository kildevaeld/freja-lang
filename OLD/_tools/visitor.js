const {
    expressions,
    statements,
    literals
} = require('./common');


exports.run = function (name) {
    [
        `pub struct ${name} {}`,
        `impl for  ${name}`
    ]
}