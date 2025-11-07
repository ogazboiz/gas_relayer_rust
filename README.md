# gas_relayer_rust

## Docker Setup

### Prerequisites
- Docker Engine 24+
- Docker Compose V2 (bundled with recent Docker Desktop releases)

### 1. Build and run everything
```bash
docker compose up --build
```

The command starts two containers:
- `db`: PostgreSQL 16 with database `relayer`
- `relayer`: the Rust service built from `bins/relayer`

Once the build finishes, the API is available at http://localhost:8080.

### 2. Environment variables
The compose file passes sensible defaults. Override them via a `.env` file in the project root or with `--env-file` / inline `APP_*` overrides as needed.

| Variable | Default | Description |
| --- | --- | --- |
| `APP_ENVIRONMENT` | `Local` | Must be `Local` or `Production`; controls runtime mode |
| `APP_PORT` | `8080` | Port the service binds to inside the container |
| `MAX_DB_CONNECTION` | `5` | Connection pool size for PostgreSQL |
| `DATABASE_URL` | `postgres://postgres:postgres@db:5432/relayer` | Connection string consumed by `sqlx` |

### 3. Useful commands
- Rebuild after code changes: `docker compose up --build relayer`
- Follow logs: `docker compose logs -f relayer`
- Apply database migrations: migrations run automatically on start; place SQL files in `crates/db/migrations`

### 4. Stopping and cleaning up
```bash
docker compose down
docker volume rm gas_relayer_rust_db_data   # optional reset of the Postgres volume
```