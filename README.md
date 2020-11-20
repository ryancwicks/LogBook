# Log Book

This program is a log book for use through a web broweser. The backend is written in Rust, and the front end is vanilla javascript. It uses a Postgres database for log storage.

## Development Setup

To get the database working, we need to install sqlite. For windows, this involves building sqlite and nasty path stuff. For linux, I installed libsqlite3-dev (Ubuntu).

Switch project to rust nightly.

```
rustup override set nightly
```

Cargo watch is a handy tool to use for this sort of work:

```
cargo install cargo-watch
cargo watch -x run
```

We also need to install the diesel-cli program.

```
cargo install diesel_cli --no-default-features --features sqlite
```

To set up the test database, do the following:

```
diesel migration run --database-url log_database.sqlite3
```

To run the tests, set up a test database:

```
diesel migration run --database-url log_database_test.sqlite3
```

From this point, I followed the getting started on the diesel website, substituting sqlite for PostGres.

## Testing

To prevent database problems, run the tests in a single thread. Their are better ways to deal with [this](https://github.com/rust-lang/rust/issues/43155), but this works.

```
cargo test -- --test-threads=1
```

