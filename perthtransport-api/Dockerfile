FROM rust:1.70.0-alpine3.18 AS chef

RUN apk add --no-cache musl-dev
RUN rustup target add x86_64-unknown-linux-musl

RUN cargo install cargo-chef
WORKDIR /app

FROM chef AS planner

COPY . .
RUN cargo chef prepare --recipe-path recipe.json --bin pta-api

FROM chef AS builder
ARG BUILD_MODE=release

COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --profile ${BUILD_MODE} --recipe-path recipe.json --target x86_64-unknown-linux-musl --bin pta-api

COPY . .
RUN cargo build --profile ${BUILD_MODE} --bin pta-api --target x86_64-unknown-linux-musl

FROM alpine:latest AS runtime
ARG BUILD_DIRECTORY=release

COPY --from=builder /app/target/x86_64-unknown-linux-musl/${BUILD_DIRECTORY}/pta-api /usr/local/bin
ENTRYPOINT ["/usr/local/bin/pta-api"]