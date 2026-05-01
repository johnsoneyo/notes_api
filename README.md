# Notes API

A small REST API for creating, reading, listing, and deleting notes.

This project is built with:

- [Axum](https://github.com/tokio-rs/axum) for HTTP routing and handlers
- [Tokio](https://tokio.rs/) as the async runtime
- [SQLx](https://github.com/launchbadge/sqlx) for async PostgreSQL access
- [Serde](https://serde.rs/) for JSON serialization and deserialization
- [dotenvy](https://github.com/allan2/dotenvy) for loading local environment variables
- [jsonwebtoken](https://github.com/Keats/jsonwebtoken) for JWT validation

## Requirements

- Rust 2024 edition toolchain
- PostgreSQL
- Keycloak or another OpenID Connect provider with RS256 JWTs
- `cargo`

## Environment

Create a `.env` file in the project root:

```env
DATABASE_URL=postgres://postgres:postgres@localhost:5432/notes_api
PORT=5432
KEYCLOAK_ISSUER=http://localhost:9080/realms/caas-dev
KEYCLOAK_AUDIENCE=account
KEYCLOAK_TOKEN_URL=http://localhost:9080/realms/caas-dev/protocol/openid-connect/token
```

`DATABASE_URL` is required and is used by SQLx to connect to PostgreSQL.

`PORT` is currently read into application config as a number. The HTTP server itself binds to `0.0.0.0:3000` in `src/main.rs`.

`KEYCLOAK_ISSUER` is used to validate JWT issuer claims and to fetch the JWKS document from:

```text
http://localhost:9080/realms/caas-dev/protocol/openid-connect/certs
```

`KEYCLOAK_AUDIENCE` is used to validate the JWT audience claim.

`KEYCLOAK_TOKEN_URL` is useful for local token requests from `http/notes.http`.

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

## Authentication

All notes routes are protected by the Axum authentication middleware in `src/main.rs`.

Every request to `/notes` must include a bearer token:

```http
Authorization: Bearer <access-token>
```

For local development with Keycloak, request a token from:

```http
POST http://localhost:9080/realms/caas-dev/protocol/openid-connect/token
Content-Type: application/x-www-form-urlencoded

grant_type=client_credentials&client_id=account&client_secret=<client-secret>
```

Copy the returned `access_token` and use it as the bearer token when calling the notes API.

## API Endpoints

### Health Check

This route is public and does not require authentication.

```http
GET /health
```

Response:

```text
ok
```

### Create Note

```http
POST /notes
Authorization: Bearer <access-token>
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
Authorization: Bearer <access-token>
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
Authorization: Bearer <access-token>
```

Returns `404 Not Found` when the note does not exist.

### Delete Note

```http
DELETE /notes/1
Authorization: Bearer <access-token>
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
  auth/              # JWT, JWKS, auth middleware, and auth models
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

The file includes a Keycloak token request and authenticated notes API calls. You can run them from an editor that supports `.http` files, such as VS Code with a REST client extension.
