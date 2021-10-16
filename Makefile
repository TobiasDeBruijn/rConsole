all: release

PWD := $(shell pwd)

JAVA := java
LIB := lib
WEB := web

LIB_OUTPUT := ${PWD}/${LIB}/.tmpbuild
WEB_OUTPUT := ${PWD}/${WEB}/.build
GRADLE_COPY_DIR := ${PWD}/${JAVA}/input-files

release:
	${MAKE} --directory=${PWD}/${LIB} release
	${MAKE} --directory=${PWD}/${WEB} 

	rm -rf ${GRADLE_COPY_DIR}
	mkdir -p ${GRADLE_COPY_DIR}/web/

	cp -r ${LIB_OUTPUT}/* ${GRADLE_COPY_DIR}
	cp -r ${WEB_OUTPUT}/* ${GRADLE_COPY_DIR}/web/

	${MAKE} --directory=${PWD}/${JAVA}

debug:
	${MAKE} --directory=${PWD}/${LIB} debug
	cp -r ${LIB_OUTPUT}/* ${GRADLE_RESOURCE_DIR}
	cp -r ${WEB_OUTPUT}/* ${GRADLE_RESOURCE_DIR}

eclipse:
	${MAKE} --directory=${PWD}/${JAVA} eclipse