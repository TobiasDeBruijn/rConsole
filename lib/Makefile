all: release

release: copy-release
debug: copy-debug

PWD := $(shell pwd)
RUST_SRC := $(shell find src/ -type f -name \*.rs)

copy-release: clean target/x86_64-unknown-linux-gnu/release/librconsole-focal.so target/x86_64-unknown-linux-gnu/release/librconsole-bionic.so target/x86_64-unknown-linux-gnu/release/librconsole-xenial.so target/aarch64-unknown-linux-gnu/release/librconsole-focal.so target/x86_64-apple-darwin/release/librconsole.dylib
	rm -rf ${PWD}/.tmpbuild
	mkdir -p ${PWD}/.tmpbuild/x86_64/darwin
	mkdir -p ${PWD}/.tmpbuild/x86_64/linux
	mkdir -p ${PWD}/.tmpbuild/aarch64/linux
	
	cp target/x86_64-unknown-linux-gnu/release/librconsole-focal.so ${PWD}/.tmpbuild/x86_64/linux/
	cp target/x86_64-unknown-linux-gnu/release/librconsole-bionic.so ${PWD}/.tmpbuild/x86_64/linux/
	cp target/x86_64-unknown-linux-gnu/release/librconsole-xenial.so ${PWD}/.tmpbuild/x86_64/linux/
	cp target/aarch64-unknown-linux-gnu/release/librconsole-focal.so ${PWD}/.tmpbuild/aarch64/linux/
	cp target/x86_64-apple-darwin/release/librconsole.dylib ${PWD}/.tmpbuild/x86_64/darwin/

copy-debug:
	rm -rf ${PWD}/.tmpbuild/
	mkdir -p ${PWD}/.tmpbuild/x86_64/linux
	
	cp target/x86_64-unknown-linux-gnu/debug/librconsole-focal.so ${PWD}/.tmpbuild/x86_64/linux/

clean:
#	rm -rf target/
#	rm -rf .tmpbuild/
	
# Sudo is required due to an issue in the docker containers
#	sudo rm -rf .build/
#	rm -rf Cargo.lock

target:
	mkdir -p target/x86_64-unknown-linux-gnu/release
	mkdir -p target/x86_64-unknown-linux-gnu/debug
	mkdir -p target/aarch64-unknown-linux-gnu/release
	mkdir -p target/x86_64-apple-darwin/release

## RELEASE
target/x86_64-unknown-linux-gnu/release/librconsole-focal.so: SRC=${PWD}/.build/x86_64/linux/focal/release
target/x86_64-unknown-linux-gnu/release/librconsole-focal.so: ${RUST_SRC} target
	mkdir -p ${SRC}
	cp -r src Cargo.toml ${SRC}

	docker run -v "${SRC}:/code/" --rm docker-registry.k8s.array21.dev/rust-amd64-focal-builder release
	cp ${SRC}/target/x86_64-unknown-linux-gnu/release/librconsole.so target/x86_64-unknown-linux-gnu/release/librconsole-focal.so

target/x86_64-unknown-linux-gnu/release/librconsole-bionic.so: SRC=${PWD}/.build/x86_64/linux/bionic/release
target/x86_64-unknown-linux-gnu/release/librconsole-bionic.so: ${RUST_SRC} target
	mkdir -p ${SRC}
	cp -r src Cargo.toml ${SRC}

	docker run -v "${SRC}:/code/" --rm docker-registry.k8s.array21.dev/rust-amd64-bionic-builder release
	cp ${SRC}/target/x86_64-unknown-linux-gnu/release/librconsole.so target/x86_64-unknown-linux-gnu/release/librconsole-bionic.so

target/x86_64-unknown-linux-gnu/release/librconsole-xenial.so: SRC=${PWD}/.build/x86_64/linux/xenial/release
target/x86_64-unknown-linux-gnu/release/librconsole-xenial.so: ${RUST_SRC} target
	mkdir -p ${SRC}
	cp -r src Cargo.toml ${SRC}

	docker run -v "${SRC}:/code/" --rm docker-registry.k8s.array21.dev/rust-amd64-xenial-builder release
	cp ${SRC}/target/x86_64-unknown-linux-gnu/release/librconsole.so target/x86_64-unknown-linux-gnu/release/librconsole-xenial.so

target/aarch64-unknown-linux-gnu/release/librconsole-focal.so: SRC=${PWD}/.build/aarch64/linux/focal/release
target/aarch64-unknown-linux-gnu/release/librconsole-focal.so: ${RUST_SRC} target
	mkdir -p ${SRC}
	cp -r src Cargo.toml ${SRC}

	docker run -v "${SRC}:/code/" --rm docker-registry.k8s.array21.dev/rust-aarch64-focal-builder release
	cp ${SRC}/target/aarch64-unknown-linux-gnu/release/librconsole.so target/aarch64-unknown-linux-gnu/release/librconsole-focal.so

target/x86_64-apple-darwin/release/librconsole.dylib: SRC=${PWD}/.build/x86_64/darwin/release
target/x86_64-apple-darwin/release/librconsole.dylib: ${RUST_SRC} target
	mkdir -p ${SRC}
	cp -r src Cargo.toml ${SRC}

	docker run -v "${SRC}:/code/" --rm docker-registry.k8s.array21.dev/rust-amd64-darwin-builder release
	cp ${SRC}/target/x86_64-apple-darwin/release/librconsole.dylib target/x86_64-apple-darwin/release/librconsole.dylib

## TODO: Windows

## DEBUG
target/x86_64-unknown-linux-gnu/debug/librconsole-focal.so: ${RUST_SRC} target
	docker run -v "${PWD}:/code/" --rm docker-registry.k8s.array21.dev/rust-amd64-focal-builder
	cp lib/target/x86_64-unknown-linux-gnu/debug/librconsole.so lib/target/x86_64-unknown-linux-gnu/debug/librconsole-focal.so