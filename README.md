# zero-to-production-in-rust

Learning Rust using [Zero To Production In Rust: An introduction to backend development](https://www.zero2prod.com/index.html?country_code=US).

## Run
* Run Docker Desktop.
* Launch the dockerized Postgres database: `./scripts/init.db.sh`
* Run the application: `cargo run`
* Look at the database: `psql -h localhost -p 5432 -U postgres`

## New Query
* Whenever a new sqlx query is added to the code, run `cargo sqlx prepare` to generate a query for the offline version of the db (used in Github actions)
