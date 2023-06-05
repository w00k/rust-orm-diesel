# rust-orm-diesel
Examples for use Rust with ORM Diesel

## First Run

Setup the database, first you need to up docker compose. 

1. Up the database, go to the root folder (when is a file docker-compose.yml).
```bash
$ docker-compose up
```

2. Enter in the project **rust-orm-diesel** and download the CLI client for postgres.
```bash 
$ cargo install diesel_cli --no-default-features --features postgres
```

3. Create the tables
```bash 
$ diesel migration run --database-url postgres://postgres:postgres@localhost:5432/orm
```

## 