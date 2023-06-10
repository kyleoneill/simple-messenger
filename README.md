# Migrations
Run the following to create and set up a database.
```
sqlx db create --database-url sqlite:database.sqlite
sqlx migrate run --database-url sqlite:database.sqlite
```
A new migration can be created with
```
sqlx migrate add <migration_name>
```

# Env
Create a .env file with the following content - `DATABASE_URL="sqlite:database.sqlite"`

# Running

## Backend

From the sm_backend folder, run `cargo run`

## Frontend

From the sm-frontend folder, run `npm run dev`
