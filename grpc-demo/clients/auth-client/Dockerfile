FROM rust:1.70 as builder

WORKDIR /opt/app
COPY . .

RUN cargo build --release

FROM debian:bullseye-slim
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /opt/app/target/release/user-client /usr/local/bin/

EXPOSE 8080 9000

CMD ["user-client"]