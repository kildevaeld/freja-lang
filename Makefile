
.PHONY: ast clean


build:
	node tools2/index.js | rustfmt > freja-parser2/src/ast.rs

ast: freja-ast/src/ast.rs

freja-ast/src/ast.rs: tools/index.js tools/template.hbs
	node $< | rustfmt > $@

clean:
	rm freja-ast/src/ast.rs

peg:
	rust-peg freja-parser2/src/grammar.rustpeg | rustfmt > freja-parser2/src/grammar2.rs

watch: watch-grammer watch-ast
	

watch-ast:
	@echo tools2/ast.js | entr make build

watch-grammer:
	@echo freja-parser2/src/grammar.rustpeg | entr make peg