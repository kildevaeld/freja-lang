
.PHONY: ast clean


ast: freja-parser2/src/ast.rs

freja-parser2/src/ast.rs: tools/index.js tools/template.hbs
	node $< > $@

clean:
	rm freja-parser2/src/ast.rs