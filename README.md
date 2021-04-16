# rConsole
Remote Console plugin for Minecraft Spigot

### Features
- Read console
- Send commands
- Simple API over TCP Sockets [docs](docs/README.md)

### Config
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

### TODO
I'm still working on a dashboard that'll nicely display this console. 