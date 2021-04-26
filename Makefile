.PHONY: testjar releasejar ghactions clean
all: releasejar

RUST_SOURCE_FILES := $(shell find librconsole/src -type f)
TS_SOURCE_FILES := $(shell find web/src/ts -type f)
SCSS_SOURCE_FILES := $(shell find web/src/scss -type f)
STATIC_WEB_CONTENT := $(shell find web -type f -name "*.html")

librconsole/target/x86_64-apple-darwin/release/librconsole.dylib: ${RUST_SOURCE_FILES}
	cd librconsole; \
		CXX=/usr/local/osx-ndk-x86/bin/o64-clang++ CC=/usr/local/osx-ndk-x86/bin/o64-clang LIBZ_SYS_STATIC=1 PKG_CONFIG_ALLOW_CROSS=1 cargo build --lib --release --target x86_64-apple-darwin
	mv librconsole/target/x86_64-apple-darwin/release/liblibrconsole.dylib librconsole/target/x86_64-apple-darwin/release/librconsole.dylib

librconsole/target/x86_64-unknown-linux-gnu/release/librconsole.so: ${RUST_SOURCE_FILES}
	cd librconsole; \
		cargo build --lib --release --target x86_64-unknown-linux-gnu
	mv librconsole/target/x86_64-unknown-linux-gnu/release/liblibrconsole.so librconsole/target/x86_64-unknown-linux-gnu/release/librconsole.so

librconsole/target/x86_64-pc-windows-gnu/release/librconsole.dll: ${RUST_SOURCE_FILES}
	cd librconsole; \
		cargo build --lib --release --target x86_64-pc-windows-gnu

testjar: librconsole/target/x86_64-unknown-linux-gnu/release/librconsole.so librconsole/target/x86_64-pc-windows-gnu/release/librconsole.dll web/dist.zip librconsole/target/x86_64-apple-darwin/release/librconsole.dylib
	chmod +x gradlew
	rm -rf ./build/resources
	./gradlew testjar

releasejar: librconsole/target/x86_64-unknown-linux-gnu/release/librconsole.so librconsole/target/x86_64-pc-windows-gnu/release/librconsole.dll web/dist.zip librconsole/target/x86_64-apple-darwin/release/librconsole.dylib
	chmod +x gradlew
	rm -rf ./build/resources
	./gradlew releasejar

ghactions: librconsole/target/x86_64-unknown-linux-gnu/release/librconsole.so librconsole/target/x86_64-pc-windows-gnu/release/librconsole.dll web/dist.zip librconsole/target/x86_64-apple-darwin/release/librconsole.dylib
	chmod +x gradlew
	rm -rf ./build/resources
	./gradlew ghActions

web/node_modules:
	cd web; \
		npm i

web/dist/dist.js: web/node_modules ${TS_SOURCE_FILES} ${SCSS_SOURCE_FILES}
	cd web; \
		npx webpack

web/dist/ansi_up.js: web/node_modules
	cp web/node_modules/ansi_up/ansi_up.js web/dist/ansi_up.js

web/dist.zip: web/dist/dist.js web/dist/ansi_up.js ${STATIC_WEB_CONTENT}
	rm -f web/dist.zip
	cd web; \
		zip -r dist.zip static/ dist/ index.html

clean:
	cd librconsole; \
		cargo clean
	cd web; \
		npm clean
		rm -f dist.zip
	./gradlew cleanEclipse clean
