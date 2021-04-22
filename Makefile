.PHONY: testjar releasejar ghactions clean
all: releasejar

RUST_SOURCE_FILES := $(shell find librconsole/src -type f)
TS_SOURCE_FILES := $(shell find web/src/ts -type f)
SCSS_SOURCE_FILES := $(shell find web/src/scss -type f)
STATIC_WEB_CONTENT := $(shell find web -type f -name "*.html")

#NPROCS = $(shell grep -c 'processor' /proc/cpuinfo)
#MAKEFLAGS += -j$(NPROCS)

librconsole/target/x86_64-unknown-linux-gnu/release/librconsole.so: ${RUST_SOURCE_FILES}
	cd librconsole; \
		cargo build --lib --release --target x86_64-unknown-linux-gnu
	mv librconsole/target/x86_64-unknown-linux-gnu/release/liblibrconsole.so librconsole/target/x86_64-unknown-linux-gnu/release/librconsole.so

librconsole/target/x86_64-pc-windows-gnu/release/librconsole.dll: ${RUST_SOURCE_FILES}
	cd librconsole; \
		cargo build --lib --release --target x86_64-pc-windows-gnu

testjar: librconsole/target/x86_64-unknown-linux-gnu/release/librconsole.so librconsole/target/x86_64-pc-windows-gnu/release/librconsole.dll web/dist.zip
	chmod +x gradlew
	rm -rf ./build/resources
	./gradlew testjar

releasejar: librconsole/target/x86_64-unknown-linux-gnu/release/librconsole.so librconsole/target/x86_64-pc-windows-gnu/release/librconsole.dll web/dist.zip
	chmod +x gradlew
	rm -rf ./build/resources
	./gradlew releasejar

ghactions: librconsole/target/x86_64-unknown-linux-gnu/release/librconsole.so librconsole/target/x86_64-pc-windows-gnu/release/librconsole.dll web/dist.zip
	chmod +x gradlew
	rm -rf ./build/resources
	./gradlew ghActions

web/node_modules:
	cd web; \
		npm i

web/dist/dist.js: $(wildcard web/node_modules) ${TS_SOURCE_FILES} ${SCSS_SOURCE_FILES}
	cd web; \
		npx webpack

web/dist/ansi_up.js: $(wildcard web/node_modules/ansi_up/ansi_up.js)
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
