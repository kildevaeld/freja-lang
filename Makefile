
.PHONY: ast clean


ast: freja-ast/src/ast.rs

freja-ast/src/ast.rs: tools/index.js tools/template.hbs
	node $< | rustfmt > $@

clean:
	rm freja-ast/src/ast.rs