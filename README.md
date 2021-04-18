# rConsole
Remote Console plugin for Minecraft Spigot

## Features
- Read console
- Send commands
- Simple API over TCP Sockets [docs](docs/README.md)
- WIP API with HTTP requests (Websockets, maybe?)

## Config
The default configuration is as follows:
```yaml
debugMode: true
listenPort: 8080
tokens:
- name: TEST
  token: ABC
  scopes:
  - READ_CONSOLE
  - SEND_COMMAND
```

- `debugMode`: Whether rConsole should print debug messages
- `listenPort`: The port on which the TCP socket server should listen
- `tokens`:  
    - `name`: The name of the token, this **must** match on the client (Case sensitive)  
    - `token`: The token itself, this **must** match on the client (Case sensitive)  
    - `scopes`:  
        - `READ_CONSOLE`: Read everything printed to the console  
        - `SEND_COMMAND`: Send commands to the server. To get the output of the command, `READ_CONSOLE` is required.  

>Note: Scope checking hasn't been implementend as of v0.1

## TODO
I'm still working on a dashboard that'll nicely display this console. 

## Building
To build this plugin you need a couple of dependencies. These are the dependencies for Linux, since that's what I use.
- gcc-mingw-w64
- gcc
- cargo
- rustup
- make
- Java 11

Rustup toolchains:
- `stable-x86_64-pc-windows-gnu`
- `stable-x86_64-unknown-linux-gnu`
Rustup targets:
- `x86_64-unknown-linux-gnu`
- `x86_64-pc-windows-gnu`

To then build the project all you'd need to do is run `make`. This will build a `releasejar`, to create a `testjar` run `make testjar`

To get the headers for the native function, for if you want the signature again, or something :)  
``javac -h . ./src/main/java/nl/thedutchmc/rconsole/webserver/Native.java``

To get the signature from other methods, if you want to access them from native code:  
``javap -s ./build/classes/java/main/nl/thedutchmc/rconsole/<Path to the class>``