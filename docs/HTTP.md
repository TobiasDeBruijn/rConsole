# HTTP API Documentation

## Console
### Full log
Path: `/console/all`
Parameters: none
Body: (x-www-form-urlencoded)
```
key = <YOUR AUTH KEY>
name = <NAME ASSOCIATED WITH KEY>
```

### Response
```jsonc
{
    "status": 200,                                      //The status, following HTTP status codes
    "logs": [                                           //This is null if `status` is not equal to 200 
        {
            "id": 0,                                    //ID of the entry, these are incremental
            "log_entry": {
                "message": "Preparing level \"world\"", //The log message
                "timestamp": 1618782780,                //Epoch timestamp
                "level": "INFO",                        //Log level, either WARN or INFO
                "thread": "Server thread"               //Thread from which the log originated
            }
        }
    ]
}
```

### Log since
Path: `/console/since`
Parameters: none
Body: (x-www-form-urlencoded)
```
key = <YOUR AUTH KEY>
name = <NAME ASSOCIATED WITH KEY>
since = ID of the log entry to start at
```

### Response
```jsonc
{
    "status": 200,                                      //The status, following HTTP status codes
    "logs": [                                           //This is null if `status` is not equal to 200 
        {
            "id": 0,                                    //ID of the entry, these are incremental
            "log_entry": {
                "message": "Preparing level \"world\"", //The log message
                "timestamp": 1618782780,                //Epoch timestamp
                "level": "INFO",                        //Log level, either WARN or INFO
                "thread": "Server thread"               //Thread from which the log originated
            }
        }
    ]
}
```