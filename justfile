export CONFIG := "config.ron"

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

tag version:
    git tag --force {{version}}
    git push --force origin {{version}}

# creates a new up and down migration
migration-new name:
    sqlx migrate add -r {{name}}

# runs all the migrations
migration-run:
    sqlx migrate run

# revert the last migration
migration-revert:
    cargo sqlx migrate revert
