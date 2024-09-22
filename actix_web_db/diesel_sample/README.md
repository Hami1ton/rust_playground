# diesel_sample

## env

- WSL2(windows11)
- rust v1.80.1


## sqlx

```
- install sqlx cli
cargo install sqlx-cli

- create .env file, and write database-url
DATABASE_URL={sqlite url}

- create db
sqlx database create 

- create migration files
sqlx migrate add -r user

- apply migration
sqlx migrate run

- drop tables
sqlx migrate revert

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



