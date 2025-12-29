# 创建 workspace
```shell
mkdir grpc-service && cd grpc-service
cargo new --lib proto
cargo new --lib server
cargo new --lib client
cargo new --bin cli-client
```


# 根目录 Cargo.toml
```shell
cat > Cargo.toml << 'EOF'
[workspace]
members = ["proto", "server", "client", "cli-client"]
resolver = "2"
EOF
```

# 运行项目
```shell

# 原来的命令
cargo run -p user-server

# 使用 cargo watch
cargo watch -x 'run -p user-server'

cargo watch -x 'run --package user-server'
cargo watch -w user-server -x 'run'
# 带有清理和重建
cargo watch -x 'clean -p user-server' -x 'run -p user-server'

# 验证
grpcurl -plaintext 127.0.0.1:50051 list
```
