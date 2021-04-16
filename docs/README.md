# API Documentation
Communication happens over TCP sockets. Encoding is UTF-8, lines are terminated with a line seperator (Platform dependent)

### Signin
The first thing you should do after connecting is signing in:
```json
{
    "path": "/login", 
    "name": "TEST", 
    "token": "ABC", 
    "scopes": [
        "READ_CONSOLE", 
        "SEND_COMMAND"
    ]
}
```
- `name`: The name defined for this Token in rConsole's `plugin.yml` (Case sensitive)
- `token`: The token defined for this Token in rConsole's `plugin.yml` (Case sensitive)
- `scopes`: A list of Strings with the scopes requested. This does not have to include all scopes permitted for this Token in `config.yml`.

#### Response
An example response would look like:
```json
{
    "status": 200,
    "message": null
}
```
- `status`: Follows HTTP status codes:
    - `200`: OK
    - `400`: Bad Request
    - `401`: Unauthorized
- `message`: A message indicating the error, if the status code is not `200`

### Subscribing to Console output
```json
{
    "path": "/subscribe", 
    "subscribeType": "CONSOLE_OUTPUT"
}
```

#### Response
An example response would look like:
```json
{
    "status": 200,
    "message": null
}
```
- `status`: Follows HTTP status codes:
    - `200`: OK
    - `400`: Bad Request
    - `401`: Unauthorized
- `message`: A message indicating the error, if the status code is not `200`

#### Console log events
Whenever something is logged to the console, and you are subscribed to console output, you'll get a message in the following format:
```jsonc
{
    "intent": "CONSOLE_LOG_EVENT",
    "message": "The message logged to console",
    "level": "The log level, INFO or WARN",
    "thread": "From which Thread the logging event came",
    "timestamp": 0  //Seconds since Janauary 1st 1970, i.e Unix Time or epoch time
}
```

### Sending a command
```json
{
    "path": "/command", 
    "command": "<The command to execute>"
}
```
The command will be executed as console. You should not include the slash. So e.g to run the `/version` command you'd send:
```json
{
    "path": "/command", 
    "command": "version"
}
```
The response will be forwarded to you as a Console log event. So you should subscribe to Console output before sending a command.

### Server shutdown
Whenever the server shuts donw, all **signed in** clients will get the following message, followed by the closing of the socket:
```json
{
    "intent": "SERVER_SHUTDOWN"
}
```

## TODO
- `ref` tag, this will be included in the packet to rConsole, and will be forwarded back untouched. This will make it easier to keep track of which packet belongs to what.
