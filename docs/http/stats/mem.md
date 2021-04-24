# Memory
Get information about Memory (RAM) usage and availability

## Memory usage
Endpoint: `/stats/mem`
Method: `POST`

Request payload: (x-www-form-urlencoded)
```
session_id: The user's session ID
```

Response:
```jsonc
{
    "status": 200,      //200 if okay, 401 if session_id is invalid
    "total_mem": 1.0,   //Total memory available for new Objects
    "free_mem": 1.0,    //Memory currently used by instantiated Objects
    "max_mem": 1.0,     //Unalocated memory, but designated for future objects
}
```
![Java memory guide](jvm_mem_explained.png)
