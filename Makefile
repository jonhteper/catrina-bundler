all: prepare tool

prepare:
	rm -r ./bin || true
	mkdir ./bin
	cp LICENSE ./bin
	cp README.md ./bin
tool:
	cargo fmt
	cargo build --release
	cp target/release/catrina-bundler ./bin/catrina
dev:
	cargo fmt
	cargo build
	cp target/debug/catrina-bundler ./bin/catrina
