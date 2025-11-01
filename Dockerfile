FROM rust:alpine AS build

WORKDIR /app

COPY Cargo.toml .
COPY src/ ./src

RUN apk add musl-dev
RUN cargo build --release

FROM alpine:latest
WORKDIR /app

# COPY config.toml .
COPY templates ./templates
RUN mkdir output
RUN mkdir music

COPY --from=build /app/target/release/cardamon /usr/bin

ENTRYPOINT ["/usr/bin/cardamon", "serve"]
