export ASSETS := "target/site"
export BIND_ADDRESS := "127.0.0.1:8080"
export RUST_LOG := "web-server=trace,actix_web=info,sqlx=info,debug"

# compile-time environment variables
export DOCKER_DEFAULT_PLATFORM := "linux/amd64"

# used by sqlx-cli when creating/running/reverting migrations
export DATABASE_URL := "postgresql://cap_hill_rust@127.0.0.1/cap_hill_rust"

# list all recipes
default:
    just --list

# run unit tests
test:
    cargo leptos test

# run watching for changes
watch:
    cargo leptos watch

# clean all build artifacts
clean:
    cargo clean

# build the container image
build-image:
    docker build -t cap-hill-rust .

# run the container image removing it when stopped
run-image:
    docker run -it --rm \
    -e BIND_ADDRESS=$BIND_ADDRESS \
    -e RUST_LOG=$RUST_LOG \
    cap-hill-rust

release tag:
    cargo test
    git tag --force {{tag}}
    git push --force origin {{tag}}
    just build-image
    docker tag cap-hill-rust:latest ghcr.io/jesusguzmanjr/cap-hill-rust:{{tag}}
    docker tag cap-hill-rust:latest ghcr.io/jesusguzmanjr/cap-hill-rust:latest
    docker push ghcr.io/jesusguzmanjr/cap-hill-rust:{{tag}}
    docker push ghcr.io/jesusguzmanjr/cap-hill-rust:latest

# creates a new up and down migration
migration-new name:
    sqlx migrate add -r {{name}}

# runs all the migrations
migration-run:
    sqlx migrate run
    psql -U cap_hill_rust -d cap_hill_rust -f init.sql

# revert the last migration
migration-revert:
    cargo sqlx migrate revert
