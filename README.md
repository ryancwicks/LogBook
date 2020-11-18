# Log Book

This program is a log book for use through a web broweser. The backend is written in Rust, and the front end is vanilla javascript. It uses a Postgres database for log storage.

## Development Setup

To get the database working, we need to install sqlite. for windows, I had to put the dll from the website on my path. We also need to install the diesel-cli program.

```
cargo install diesel_cli --no-default-features --features sqlite
echo "log_database.sqlite3" > .env
```

From this point, I followed the getting started on the diesel website, substituting sqlite for PostGres.

