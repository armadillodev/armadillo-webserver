# syntax = docker/dockerfile:experimental
FROM rust:1.44-stretch AS builder
WORKDIR /app
COPY . .
RUN --mount=type=cache,target=/usr/local/cargo/registry \
	--mount=type=cache,target=/app/target \
	cargo install --path .

FROM alpine:3.7
RUN apk add --no-cache libpq
COPY --from=builder /usr/local/cargo/bin/armadillo-webserver /usr/local/bin/armadillo-webserver
CMD ["armadillo-webserver"]
