# syntax = docker/dockerfile:experimental
FROM rust:1.45-stretch AS builder
WORKDIR /app
COPY . .
RUN --mount=type=cache,target=/usr/local/cargo/registry \
	--mount=type=cache,target=/app/target \
	cargo install --path ./armadillo-server

FROM debian:buster-slim
RUN apt-get update && apt-get install -y \
	libpq-dev && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/armadillo-server /usr/local/bin/armadillo-webserver
CMD ["armadillo-webserver"]
