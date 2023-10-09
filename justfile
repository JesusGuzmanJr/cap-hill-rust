export BIND_ADDRESS := "127.0.0.1:8080"
export RUST_LOG := "web-server=trace,actix_web=info,debug"

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