FROM rust:1-slim-trixie as builder
COPY . .
RUN cargo build --release
FROM debian:trixie-slim
COPY --from=builder ./target/release/rust_echo .
EXPOSE 8080
ENTRYPOINT ["/rust_echo"]
