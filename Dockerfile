FROM rust:1-slim as builder
COPY . .
RUN cargo build --release
FROM debian:bullseye-slim
COPY --from=builder ./target/release/rust_echo .
EXPOSE 8080
ENTRYPOINT ["/rust_echo"]
