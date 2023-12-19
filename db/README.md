# Install
Install `sqlx_cli` on Ubuntu:
```
# Dependencies
sudo apt install libpq-dev

# Install sqlx
cargo install sqlx-cli --no-default-features --features native-tls,postgres
```

# Setup sqlx
All commands require that a database url is provided. This can be done either with the --database-url command line option or by setting DATABASE_URL, either in the environment or in a .env file in the current working directory.

```
echo DATABASE_URL=postgres://username:password@localhost/db_name > .env
```

# Run migrations
At the directory contains this README. Run: 
```
sqlx migrate run

# Or: 
sqlx migrate run --database-url postgres://username:password@localhost/db_name
```

# Sqlx offline
Double check your .env file.
- SQLX_OFFLINE: true, when you want to generate sqlx files for Docker build
- SQLX_OFFLINE: false, when you want to develop and check query type error
Run:
```
cd /lib/db
cargo sqlx prepare
```
It will produce queries for macros query!, query_as! at .sqlx folder. Commit changes if any.
Remember to run this whenever you write new query! Otherwise, build in Docker image will be failed.