# syntax=docker/dockerfile:experimental

FROM rust:1.80 as build

RUN apt-get update && apt-get install -y cmake
WORKDIR /app

COPY . .

RUN cd /app/server && RUST_BACKTRACE=1 cargo build --release

RUN mkdir -p /build-out

RUN cp target/release/server /build-out/

FROM ghcr.io/axieinfinity/foundry:cast@sha256:6bb3947cb0875b4f2c91e79d34f48135db356b0fa7da329613c01e772c1b6e7c as foundry-client

# Release stage - Ubuntu Container
FROM ubuntu:22.04

ENV DEBIAN_FRONTEND=noninteractive
RUN apt-get update && apt-get -y install ca-certificates libssl-dev libpq-dev gcc openssl && rm -rf /var/lib/apt/lists/*

COPY --from=foundry-client /opt/foundry/out/forge /usr/local/bin/forge
COPY --from=foundry-client /opt/foundry/out/cast /usr/local/bin/cast

COPY --from=build /build-out/server /

CMD ["/server"]
