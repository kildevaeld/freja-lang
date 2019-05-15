
.PHONY: ast clean build release


build: ast
	cargo build

ast: freja-parser/src/ast.rs

freja-parser/src/ast.rs: tools/index.js tools/template.hbs
	node $< | rustfmt > $@

clean:
	rm freja-ast/src/ast.rs

release:
	cargo build --release
	strip target/release/freja