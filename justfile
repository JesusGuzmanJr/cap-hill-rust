export BIND_ADDRESS := "127.0.0.1:8080"
export RUST_LOG := "web-server=trace,actix_web=info,debug"
export ASSETS_DIR := "assets"

# compile-time environment variables
export DOCKER_DEFAULT_PLATFORM := "linux/amd64"

# list all recipes
default:
    just --list

# build the container image
build-image:
    docker build -t cap-hill-rust .

# run the container image removing it when stopped
run-image:
    docker run -it --rm cap-hill-rust

# build and run locally
watch:
     cargo watch -x run

release tag:
    cargo test
    git tag --force {{tag}}
    git push --force origin {{tag}}
    just build-image
    docker tag cap-hill-rust:latest ghcr.io/jesusguzmanjr/cap-hill-rust:{{tag}}
    docker push ghcr.io/jesusguzmanjr/cap-hill-rust:{{tag}}
