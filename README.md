# EchoR

A small CLI that echoes any post requests it receives.

## Example Usage

EchoR will listen for requests to `localhost:7000/post` by default 

A different path and port can be specified using `--path` and `--port` args as shown in the following example:
```
./echor --path /mypath --port 8080
```

This will listen for requests on `localhost:8080/mypath`.

## TODO

* Take a list of paths to listen on
* accept different http methods