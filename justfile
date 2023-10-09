export BIND_ADDRESS := "127.0.0.1:8080"
export RUST_LOG := "web-server=trace,actix_web=info,debug"
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
    git tag {{tag}}
    git push origin {{tag}}
    just build-image
    docker tag cap-hill-rust:latest cap-hill-rust:{{tag}}
    docker tag cap-hill-rust:{{tag}} ghcr.io/jesusguzmanjr/cap-hill-rust:{{tag}}
    docker push ghcr.io/jesusguzmanjr/cap-hill-rust:{{tag}}