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
echo "log_database.sqlite3" > .env
```

From this point, I followed the getting started on the diesel website, substituting sqlite for PostGres.

