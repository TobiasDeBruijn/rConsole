all: build

PWD := $(shell pwd)

build:
	mkdir -p ${PWD}/.build
	touch ${PWD}/WEBFILE
	zip -r web.zip WEBFILE
	mv ${PWD}/web.zip ${PWD}/.build/