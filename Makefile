all: prepare format tool

prepare:
	rm -r ./bin || true
	mkdir ./bin
	mkdir ./bin/lib
	mkdir ./bin/lib/default
	cp LICENSE ./bin
	cp README.md ./bin
format:
	cd ./rust/catrina && cargo fmt
tool:
	cd ./rust/catrina/src && cargo build --release
	cp ./rust/catrina/target/release/catrina ./bin/
dev:
	cd ./rust/catrina/src && cargo build
	cp ./rust/catrina/target/debug/catrina ./bin/