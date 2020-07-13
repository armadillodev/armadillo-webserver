# syntax = docker/dockerfile:experimental
FROM rustlang/rust:nightly AS builder
WORKDIR /app
COPY . .
RUN --mount=type=cache,target=/usr/local/cargo/registry \
	--mount=type=cache,target=/app/target \
	cargo install --path .

FROM ubuntu:18.04
RUN apt-get update && apt-get install -y \
	libsqlite3-dev
COPY --from=builder /usr/local/cargo/bin/armadillo-webserver /usr/local/bin/armadillo-webserver
COPY ./db/records.db /data/db/records.db
CMD ["armadillo-webserver"]
