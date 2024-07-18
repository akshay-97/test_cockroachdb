FROM rust:slim-bookworm as builder

WORKDIR /
COPY . .

RUN apt-get update \
  && apt-get install -y libpq-dev libssl-dev pkg-config

RUN cargo build --release

FROM debian:bookworm-slim

ARG BIN_DIR=/bin
ARG BINARY_NAME=decision

EXPOSE 7777

COPY --from=builder /target/release/cockroach_ /ex

WORKDIR /

CMD ["./ex"]
