# diesel_sample

## env

- WSL2(windows11)
- rust v1.80.1
- diesel cli v2.2.4 (Supported Backends: sqlite)
- sqlite3 v3.37.2

## diesel

```
- install sqlite (これやってないと、diesel_cliいれるとき落ちる)
sudo apt-get install libsqlite3-dev sqlite3

- install diesel_cli
cargo install diesel_cli --no-default-features --features sqlite

- create .env file, and write database-url
DATABASE_URL=./sample.db

- create db
diesel setup

- create migration files
diesel migration generate user

- apply migration
diesel migration run

- drop tables
diesel migration revert

```

## sqlite3

```
- connect
sqlite3 sample.db

- select users
select * from user;

- disconnect
.exit
```

## links

sqlite
https://www.sqlite.org/

diesel
https://diesel.rs/guides/getting-started

actix-web
https://actix.rs/docs/databases/
