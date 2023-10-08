# syntax=docker/dockerfile:1

FROM rust:latest

RUN cargo install cargo-leptos --version 0.1.11

WORKDIR /build

COPY . .

RUN --mount=type=cache,target=/root/.cargo/* \
    --mount=type=cache,target=/build/target \
    cargo leptos build --release \
    && mv /build/target/site /site \
    && mv /build/target/server/release/web-server /web-server

FROM rust:slim
COPY --from=0 /web-server /web-server

ENTRYPOINT ["/web-server"]