all: prepare tool

native: prepare tool

prepare:
	rm -r ./bin || true
	mkdir ./bin
	cp LICENSE ./bin
	cp README.md ./bin
tool:
	cargo fmt
	cargo build --release
	cp target/release/catrina ./bin/catrina
dev:
	cargo fmt
	cargo build
	cp target/debug/catrina ./bin/catrina

all-targets:
	sh ./all-targets.sh

