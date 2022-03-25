```
protoc proto/core/error.proto proto/user/user-microservice.proto -I. --go_out=:pkg
```

protoc proto/core/error.proto proto/user/user-microservice.proto -I. --go_out=pkg --go_opt=paths=source_relative --go-grpc_out=pkg --go-grpc_opt=paths=source_relative