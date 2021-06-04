FROM rust:1-slim as builder
COPY Cargo.toml .
COPY Cargo.lock .
RUN mkdir src
RUN echo "fn main() {}" > src/main.rs
RUN cargo build --release
COPY . .
RUN cargo build --release
FROM debian:buster-slim
COPY --from=builder ./target/release/rust_echo .
EXPOSE 8080
ENTRYPOINT ["rust_echo"]
