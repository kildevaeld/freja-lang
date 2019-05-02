
.PHONY: ast clean


build:
	node tools/index.js | rustfmt > freja-parser/src/ast.rs

ast: freja-ast/src/ast.rs

freja-ast/src/ast.rs: tools/index.js tools/template.hbs
	node $< | rustfmt > $@

clean:
	rm freja-ast/src/ast.rs

peg:
	rust-peg freja-parser/src/grammar.rustpeg | rustfmt > freja-parser/src/grammar2.rs

watch: watch-grammer watch-ast
	

watch-ast:
	@echo tools/ast.js | entr make build

watch-grammer:
	@echo freja-parser/src/grammar.rustpeg | entr make peg