FROM arm64v8/rust:1.75.0-alpine AS chef

RUN apk add --no-cache musl-dev
RUN rustup target add aarch64-unknown-linux-musl

RUN cargo install cargo-chef
WORKDIR /app

FROM chef AS planner

RUN apk add --no-cache openssl-dev pkgconfig
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
ARG BUILD_MODE=release

COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --profile ${BUILD_MODE} --recipe-path recipe.json --target aarch64-unknown-linux-musl

COPY . .
RUN cargo build --frozen --profile ${BUILD_MODE} --bin perthtransport-worker --target aarch64-unknown-linux-musl

FROM arm64v8/alpine:latest AS runtime
ARG BUILD_DIRECTORY=release

COPY --from=builder /app/target/aarch64-unknown-linux-musl/${BUILD_DIRECTORY}/perthtransport-worker /usr/local/bin
ENTRYPOINT ["/usr/local/bin/perthtransport-worker"]
