# rConsole
Remote Console plugin for Minecraft Spigot

**This plugin is under heavy development, and is still in beta**

## Features
- Read console
- Send commands
- Simple API over TCP Sockets [docs](docs/README.md)
- Feature-rich api via HTTP [docs](docs/README.md)
  - Get Console logs
  - Execute commands
  - Server metrics like CPU usage and memory usage
- Fast and intuitive web frontend

## Platform support
Because I use Rust code, this plugin will not work on all operating systems and architectures.  
At the moment the following is supported:
- x86_64 Linux
- x86_64 Windows

I am working on support for aarch64 (ARM 64-bit, Raspberry Pi 4 and up) and armhf (ARM 32, Raspberry Pi 3 and lower) support, though this takes some work.

MacOS will likely never be supported. I don't think it is possible to easily cross-compile from Linux to MacOS.  
If you know how to achieve this with the default Rust toolchains, please open an [issue](https://github.com/TheDutchMC/rConsole/issues/new)

## On the agenda
- File browser
- Different permission levels for the webinterface

## Config
The default configuration is as follows:
```yaml
# rConsole general configuration
# Should debug logs be printed to the console
debugMode: true

# The port on which the TCP server will listen
listenPort: 8080

# Should we also load the web server
# Most users should keep this set to true
useWebServer: true

# Authentication tokens for the TCP server
tokens:
- name: TEST
  token: ABC
  scopes:
  - READ_CONSOLE
  - SEND_COMMAND

# This bit of configuration is specific to librconsole
# On which port should the webserver listen. This is also the port you'll use in the browser to access the web-interface
librconsolePort: 8090

# The password pepper to be used. This MUST not be longer than 16 characters. You should not leave this at the default
# This is used to encrypt passwords
pepper: mrsXlQy9friisbeW

# If you are not accessing the frontend through localhost, but through a domain e.g, enter it here
# e.g: https://rconsole.example.com
# You MUST not include a trailing slash, so https://rconsole.example.com/ is WRONG
# Keep in mind that rConsole does NOT do SSL termination, so if you want HTTPS, you must use a reverse proxy like NGINX
baseUrl: http://localhost:8090
```

## Building
To build this plugin you need a couple of dependencies. These are the dependencies for Linux, since that's what I use.
- gcc-mingw-w64
- gcc
- cargo
- rustup
- make
- openjdk-11-jdk-headless
- npm

Rustup toolchains:  
- `stable-x86_64-pc-windows-gnu`  
- `stable-x86_64-unknown-linux-gnu`  
Rustup targets:  
- `x86_64-unknown-linux-gnu`  
- `x86_64-pc-windows-gnu`  

To then build the project all you'd need to do is run `make`. This will build a `releasejar`, to create a `testjar` run `make testjar`.  
a `releasejar` will be outputted to `$projectRoot/releases`, a `testjar` will be outputted to `$projectRoot/server/plugins`

To get the headers for the native function, for if you want the signature again, or something :)  
``javac -h . ./src/main/java/nl/thedutchmc/rconsole/webserver/Native.java``

To get the signature from other methods, if you want to access them from native code:  
``javap -s ./build/classes/java/main/nl/thedutchmc/rconsole/<Path to the class>``