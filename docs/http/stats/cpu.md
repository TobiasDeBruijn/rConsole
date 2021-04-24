# CPU
Get information about the CPU and CPU usage

## CPU Usage (load)
Endpoint: `/stats/cpu/load`
Method: `POST`

Request payload: (x-www-form-urlencoded)
```
session_id: The user's session ID
```

Request response;
```jsonc
{
    "status": 200, //200 if okay, 401 if session_id is invalid
    "load": {              //For linux, see alsso: https://man7.org/linux/man-pages/man3/getloadavg.3.html
        "one": 1.0,        //On *nix: Average over 1 minute. On Windows: CPU utilization with a 500ms sampling period, or -1.0 if an error occurred
        "five": 1.0,       //On *nix: Average over 5 minutes. -1.0 on Windows (Not supported)
        "fifteen": 1.0     //On *nix: Average over 15 minutes. -1.0 on Windows (Not supported)
    }
}
```
