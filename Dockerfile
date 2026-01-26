FROM rust:1.76 as builder

WORKDIR /app

RUN mkdir -p interpreter/src listener/src && \
    echo "fn main() {}" > interpreter/src/main.rs && \
    echo "fn main() {}" > listener/src/main.rs

COPY Cargo.toml Cargo.lock ./
COPY interpreter/Cargo.toml interpreter/
COPY listener/Cargo.toml listener/

RUN cargo build --release --workspace

COPY interpreter/src interpreter/src
COPY listener/src listener/src

RUN touch interpreter/src/main.rs listener/src/main.rs && \
    cargo build --release --workspace

FROM debian:bookwork-slim
WORKDIR /app

RUN apt-get update && apt-get install -y ca-certificate openssl && \
    rm -rf /var/lib/apt/lists/*

ARG BINARY_NAME

COPY --from=builder /app/target/release/${BINARY_NAME} /app/worker

CMD ["/app/worker"]
