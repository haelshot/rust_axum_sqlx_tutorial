# Backend with :
- Rust-Axum
- SQLx
- PostgreSQL

# Configuring SQLx

### create a new .env that looks something like:
DATABASE_URL=postgresql://username:password@host/db_name

### Then run on terminal
```bash
sqlx database create
```
### To create migrations run the following command
```bash
sqlx migrate add create_table_name_table
```
The previous command creates the new file TIMESTAMP_create_table_name_table.sql where TIMESTAMP is the Unix timestamp of the migration. Then write SQL to create the new table in the sql file created by this command.
EXAMPLE:

```sql
CREATE TABLE IF NOT EXISTS app_users (
    id BIGSERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    email TEXT NOT NULL,
);
```

### Then run migrations with the following command
```bash
sqlx migrate run
```

The command below triggers recompilation after adding possible later migrations:
```bash
sqlx migrate build-script
```

### Then in Cargo.toml file add the following
```toml
[dependencies]
dotenvy = "0.15.7"
sqlx = { version = "0.6.3", features = ["postgres", "macros", "runtime-tokio-rustls"] }
```