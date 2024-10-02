# SQLX `query*!` macros bug

## Steps to reproduce the bug

Prepare your own DATABASE_URL, default is `postgres://postgres:postgres@localhost:5432`

At this point you can even see the error from `service_1` saying

```
error returned from database: database "test_db_2" does not exist
```

Or from `service_2` saying

```
error returned from database: database "test_db_1" does not exist
```

This should not happen because `service_1` and `service_2` are using `test_db_1` and `test_db_2` respectively.

You can further perform migration and see the error

```bash
cd services/service_1
cargo sqlx database create
cargo sqlx migrate
```

```bash
cd services/service_2
cargo sqlx database create
cargo sqlx migrate
```

I found a workaround for this bug is to set env inside Cargo.toml file of each service directory.
