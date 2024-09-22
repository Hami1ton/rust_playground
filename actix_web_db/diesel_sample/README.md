# diesel_sample

## env

- WSL2(windows11)
- rust v1.80.1
- diesel cli v2.2.4 (Supported Backends: sqlite)


## sqlx

```
- install sqlite
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
diesel migration redo

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



