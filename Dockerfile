# syntax=docker/dockerfile:1

# this container provides statically built dependencies
# that Rust can link to when building the static binary
# https://github.com/clux/muslrust
FROM clux/muslrust:nightly-2023-10-01 as builder

RUN apt-get update && apt-get install musl-tools -y
RUN rustup target add x86_64-unknown-linux-musl
RUN CARGO_BUILD_TARGET=x86_64-unknown-linux-musl cargo install cargo-leptos --version 0.2.0

LABEL org.opencontainers.image.description Cap Hill Rust web server image

WORKDIR /build

COPY . .

RUN --mount=type=cache,target=/root/.cargo/* \
    --mount=type=cache,target=/build/target \
    cargo leptos build --release \
    && mv /build/target/site /site \
    && mv /build/target/server/release/cap-hill-rust /cap-hill-rust

FROM scratch
COPY --from=0 /cap-hill-rust /cap-hill-rust
COPY --from=0 /build/assets /assets

ENV ASSETS_DIR=/assets

ENTRYPOINT ["/cap-hill-rust"]