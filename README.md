# Notes API

A small REST API for creating, reading, listing, and deleting notes.

This project is built with:

- [Axum](https://github.com/tokio-rs/axum) for HTTP routing and handlers
- [Tokio](https://tokio.rs/) as the async runtime
- [SQLx](https://github.com/launchbadge/sqlx) for async PostgreSQL access
- [Serde](https://serde.rs/) for JSON serialization and deserialization
- [dotenvy](https://github.com/allan2/dotenvy) for loading local environment variables

## Requirements

- Rust 2024 edition toolchain
- PostgreSQL
- `cargo`

## Environment

Create a `.env` file in the project root:

```env
DATABASE_URL=postgres://postgres:postgres@localhost:5432/notes_api
PORT=5432
```

`DATABASE_URL` is required and is used by SQLx to connect to PostgreSQL.

`PORT` is currently read into application config as a number. The HTTP server itself binds to `0.0.0.0:3000` in `src/main.rs`.

## Database

The API expects a `notes` table:

```sql
CREATE TABLE notes (
    id SERIAL PRIMARY KEY,
    title TEXT NOT NULL,
    content TEXT NOT NULL
);
```

The migration is stored in:

```text
migrations/20240428123456_create_notes_table.sql
```

If you use SQLx CLI, you can run migrations with:

```bash
cargo install sqlx-cli --no-default-features --features postgres
sqlx migrate run
```

## Running The App

Start PostgreSQL, make sure `DATABASE_URL` is set, then run:

```bash
cargo run
```

The API will be available at:

```text
http://localhost:3000
```

## API Endpoints

### Create Note

```http
POST /notes
Content-Type: application/json

{
  "title": "Second note",
  "content": "This is my second note"
}
```

Response:

```json
{
  "id": 1,
  "title": "Second note",
  "content": "This is my second note"
}
```

### List Notes

```http
GET /notes
```

Response:

```json
[
  {
    "id": 1,
    "title": "Second note",
    "content": "This is my second note"
  }
]
```

Notes are returned in descending `id` order.

### Get Note By ID

```http
GET /notes/1
```

Returns `404 Not Found` when the note does not exist.

### Delete Note

```http
DELETE /notes/1
```

Response:

```text
deleted
```

## Project Structure

```text
src/
  main.rs            # Application entry point and Axum server startup
  app.rs             # Shared application state and database pool setup
  routes/notes.rs    # Notes route definitions
  handlers/notes.rs  # HTTP handlers for note actions
  db/notes.rs        # SQLx queries for notes
  models/note.rs     # Request and response data models
  state/             # App state structs for config, database, and HTTP client
```

## Testing

Run the test suite:

```bash
cargo test
```

This project uses SQLx compile-time checked query macros. That means `cargo check`, `cargo test`, and `cargo build` may need access to the database described by `DATABASE_URL` unless SQLx offline metadata is generated.

## Example Requests

Example HTTP requests are available in:

```text
http/notes.http
```

You can run them from an editor that supports `.http` files, such as VS Code with a REST client extension.
