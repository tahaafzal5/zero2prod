# zero-to-production-in-rust

Learning Rust using [Zero To Production In Rust: An introduction to backend development](https://www.zero2prod.com/index.html?country_code=US).

## Run
* Run Docker Desktop.
* Launch the dockerized Postgres database: `./scripts/init.db.sh`
* Run the application: `cargo run`
* Look at the database: `psql -h localhost -p 5432 -U postgres`

## New Query
* Whenever a new sqlx query is added to the code, run `cargo sqlx prepare` to generate a query for the offline version of the db (used in Github actions)

## Add a new migration
`sqlx migrate add <migration name>` 

## Apply the migrations to the local, development database
`sqlx migrate run` while the docker container is running.

## Migrate the production database on Digital Ocean
`DATABASE_URL=DIGITAL-OCEAN-DB-CONNECTION-STRING sqlx migrate run`
