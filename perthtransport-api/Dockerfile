FROM arm64v8/rust:1.70.0-alpine3.18 AS chef

RUN apk add --no-cache musl-dev
RUN rustup target add aarch64-unknown-linux-musl

RUN cargo install cargo-chef
WORKDIR /app

FROM chef AS planner

COPY . .
RUN cargo chef prepare --recipe-path recipe.json --bin perthtransport-api

FROM chef AS builder
ARG BUILD_MODE=release

COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --profile ${BUILD_MODE} --recipe-path recipe.json --target aarch64-unknown-linux-musl --bin perthtransport-api

COPY . .
RUN cargo build --profile ${BUILD_MODE} --bin perthtransport-api --target aarch64-unknown-linux-musl

FROM arm64v8/alpine:latest AS runtime
ARG BUILD_DIRECTORY=release

COPY --from=builder /app/target/aarch64-unknown-linux-musl/${BUILD_DIRECTORY}/perthtransport-api /usr/local/bin
ENTRYPOINT ["/usr/local/bin/perthtransport-api"]
