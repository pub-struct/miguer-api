FROM rust:1.83.0-slim as builder

WORKDIR /usr/src/

COPY . .

RUN cargo build --release

FROM debian:bookworm-slim

WORKDIR /usr/app

COPY --from=builder /usr/src/config config
COPY --from=builder /usr/src/target/release/miguer_api-cli miguer_api-cli

ENTRYPOINT ["/usr/app/miguer_api-cli start --environment production"]
