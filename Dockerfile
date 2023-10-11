# syntax=docker/dockerfile:1

FROM messense/rust-musl-cross:x86_64-musl

LABEL org.opencontainers.image.description Cap Hill Rust web server image

RUN rustup target add x86_64-unknown-linux-musl

WORKDIR /build

COPY . .

RUN --mount=type=cache,target=/root/.cargo/* \
    --mount=type=cache,target=/build/target \
    cargo build --release \
    && mv /build/target/x86_64-unknown-linux-musl/release/web-server /web-server

FROM scratch
COPY --from=0 /web-server /web-server

ENTRYPOINT ["/web-server"]