all: prepare format tool

prepare:
	rm -r ./bin || true
	mkdir ./bin
	cp LICENSE ./bin
	cp README.md ./bin
format:
	cargo fmt
tool: format
	cd src && cargo build --release
	cp target/release/catrina-bundler ./bin/catrina
dev: format
	cd src && cargo build
	cp target/debug/catrina-bundler ./bin/catrina
