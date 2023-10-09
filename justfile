# used by the web-server during development
export BIND_ADDRESS := "127.0.0.1:8080"

build-image:
     docker build -t cap-hill-rust .

run-image:
    docker run -it --rm cap-hill-rust

watch:
     cargo watch -x run