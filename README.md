# rust_backend

Simple backend application written in rust.

## Project setup

1) First make sure to install all required dependencies:
```
bash scripts/install_deps.sh
```
2) After dependenies are installed make sure they are visible in current scope.
```
export PATH="~/.cargo/bin/:$PATH"
```
This may vary depending on the system.

3) Make sure to have postgres database instance running. 
```
bash scripts/init_db.sh
```
This scripts contains default values for postgres database. You can override them by setting 
corresponding env variables.
```
DB_USER - #default postgres
DB_PASSWORD - #default password
DB_NAME - #default booklib
DB_PORT - #default 5432
```

4) Export `DATABASE_URL` env variables which is required by rust analyzer to make sure `sqlx` queries are 
correct. Example:
```
 export DATABASE_URL=postgres://postgres:password@0.0.0.0:5432/booklib
```

5) Finally run rust backend:
```
cargo run
```

## Running Test

To execute tests simply run:
```
cargo test
```
