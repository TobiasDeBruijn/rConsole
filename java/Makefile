GRADLEW := ./gradlew
PWD := $(shell pwd)

all: clean ${PWD}/.build/rconsole.jar

clean:
	rm -rf ${PWD}/build/
	rm -rf ${PWD}/.gradle/
	rm -rf ${PWD}/.build

${PWD}/.build/rconsole.jar:
	${GRADLEW} releaseJar
	mkdir -p ${PWD}/.build/
	cp ${PWD}/releases/rConsole-RELEASE.jar ${PWD}/.build/rconsole.jar

eclipse:
	${GRADLEW} eclipse