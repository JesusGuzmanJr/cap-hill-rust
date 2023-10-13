# Cap Hill Rust

This is the source code repo for the Cap Hill Rust [homepage](https://caphillrust.com).

### Postgres setup
The app uses postgres as the database. To set it up for local development, follow these steps:

1. Create a database within postgres. Type `sudo -u postgres psql` (or just `psql` on Mac) in your terminal to enter the postgres interpreter as the postgres user. This user has admin permissions to create more users.
2. Run `\du` to describe users.
3. Create a new user (called roles in postgres) with:

    `create role cap_hill_rust with login;`

4. Run `\du` to view the users again. You should see the new user.
4. Create the database:

    `create database cap_hill_rust with owner cap_hill_rust;`

6. Logout of psql:

    `\q`

## Contributing

If you'd like to contribute to the site, please open a PR with your changes.