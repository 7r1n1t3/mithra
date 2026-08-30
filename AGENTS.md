# AGENTS.md

## Project overview

Mithra is a self-hosted TOTP vault. It's a monorepo with two parts shipped as a single Docker
image: a Rust/Actix-web backend (`backend/`) that serves both the JSON API and the built frontend
static files, and a SvelteKit frontend (`frontend/`) built as a static SPA.

## Commands

### Full stack (Docker)

```bash
openssl rand -base64 64   # generate SESSION_SECRET
cp .env.example .env      # then fill in POSTGRES_*/SESSION_SECRET
docker compose up --build # app on :8080, plus postgres:17 and redis:8.8
```

### Backend (`backend/`, Rust/Actix-web + sqlx/Postgres)

```bash
cargo build
cargo run     # needs DATABASE_URL, REDIS_URL, SESSION_SECRET env vars set
              # (bring up just the datastores with: docker compose up db redis)
cargo test
cargo fmt && cargo clippy
```

Schema changes go through **diesel migrations**, which are the only part of diesel still in the
project — it is the migration runner, not the query layer. Each migration is a directory
`backend/migrations/yyyy-mm-dd-desc/` holding `up.sql` and `down.sql`; `src/migrations.rs` bakes
them into the binary with `embed_migrations!("migrations")` and `main.rs` awaits
`migrations::run(&database_url)` on startup, before the sqlx pool is built, so a fresh database
self-initialises on first boot and the runtime image ships no `.sql` files. Diesel tracks what it
has applied in a `__diesel_schema_migrations` table it creates itself, so startup is a no-op once
the schema is current. Note it records the directory name with the punctuation stripped
(`2026-08-30-initial-schema` → `20260830initialschema`), and orders migrations by that string —
keep the `yyyy-mm-dd` prefix so ordering stays chronological.

Adding a migration means creating the directory and both `.sql` files by hand (there is no
`diesel_cli` dependency here); `down.sql` is never run by the app, it exists so a revert is
possible with `diesel migration revert` if you install the CLI. Rust-side structs and enums in
`backend/src/auth/models.rs` are kept in sync with the schema manually — nothing is generated.

The connection is async: `migrations::run` establishes a `diesel_async::AsyncPgConnection` and
hands it to `AsyncConnectionWrapper`, since `MigrationHarness` is a blocking trait — the wrapper
drives the connection's futures internally, so the harness call itself is wrapped in
`spawn_blocking` and must never run directly on an async worker. Using `diesel-async` (with
diesel's `postgres_backend` feature rather than `postgres`) keeps the pure-Rust tokio-postgres
path, so the binary links no libpq and needs no extra system packages in either Docker stage.

Because migrations run in-process, Postgres needs no init scripts: `docker-compose.yml` mounts
nothing into `/docker-entrypoint-initdb.d`; the Dockerfile only has to `COPY backend/migrations`
into the build stage so `embed_migrations!` can find them.

Queries are written with sqlx's *runtime* API (`sqlx::query`, `query_as`, `query_scalar` with
`.bind(...)`), not the compile-time-checked `query!` macros — so there is no `.sqlx` offline
cache and no database is needed to build. The flip side is that SQL typos and column/type
mismatches only surface at runtime.

All primary keys are `integer ... GENERATED ALWAYS AS IDENTITY` and foreign keys are `integer`;
`auth::models::ID` (`i32`) is the single Rust-side alias for them. Postgres enum types are
mirrored 1:1 by `#[derive(sqlx::Type)]` enums in `auth/models.rs`, each tagged with
`#[sqlx(type_name = "...", rename_all = "snake_case")]` matching the SQL type name.

### Frontend (`frontend/`, SvelteKit)

Local dev uses `bun` (see `bun.lock`); the Docker build uses `npm ci`/`npm run build`.

```bash
bun install
bun run dev      # vite dev server, proxies /api/* to http://127.0.0.1:8080
bun run build
bun run check    # svelte-kit sync && svelte-check
```

## Architecture

### Backend request flow

On startup `main.rs` first applies any pending diesel migrations (see above), then builds the
sqlx pool. It then wires up an Actix `App` with, in order: access-log middleware,
`SessionMiddleware` (Redis-backed via `actix-session`/`RedisSessionStore`), `AppState` (the pool,
built with `PgPoolOptions::max_connections(5)`), the `/api` route tree, and a static-file
fallback that serves `./build` (the compiled frontend) with `200.html` as both the index and the
SPA fallback — i.e. this is a single Actix server serving an API and an SPA, not two separately
deployed services.

Module layout under `backend/src/`:

- `routes/` — Actix handlers only, thin. `routes::configure` mounts `/api`; `routes/api/mod.rs`
  mounts `/api/register` and `/api/signin`. (`routes/api/register/code.rs` is declared but still
  empty, and nothing is mounted from it.)
- `auth/` — shared types: `models` (the DB-shaped structs `User`, `Session`, `LoginAttempt`, the
  `ID` alias, and the `sqlx::Type` enums mirroring the Postgres enum types), `dto`
  (request/response JSON shapes), `extractor` (`AuthedUser`, an Actix `FromRequest` that pulls
  the user id out of the session — see below), `error` (`DatabaseError`, wrapping `sqlx::Error`
  and `SessionInsertError`).
- `services/` — business logic called from route handlers, and where all runtime SQL lives:
  `auth.rs` (credential verification, user/session registration, login-attempt logging),
  `password.rs` (Argon2 hashing/verification via the `argon2` crate). `verify_credentials`
  returns `Result<(ID, bool), sqlx::Error>` — `(-1, false)` is the sentinel for "no such user",
  `(id, false)` means bad password.
- `state.rs` — `AppState { pgpool: PgPool }`, cloned into each worker and injected via
  `web::Data`.
- `migrations.rs` — the embedded diesel migrations (`MIGRATIONS`) and `run(database_url)`, called
  once from `main` before anything else touches the database.

Auth model: sessions are dual-written — once to Postgres (`sessions` table, for audit/history)
and once into the Redis-backed Actix session cookie (`services::auth::cache_session`, key
`"session"`). Per-request authorization (`AuthedUser` extractor) only reads the Redis-cached
session, never the DB, so it's fast but any DB-side session revocation needs a separate
Redis-side check if that's ever added. The first user ever registered is automatically given
`UserRole::Owner` (checked via `get_number_users(...) == 0` in the register handler); everyone
else defaults to `UserRole::User`.

### Frontend

SvelteKit with `adapter-static` and `ssr = false` (see `+layout.server.js`) — it's a pure SPA;
routing/rendering all happens client-side, and the backend serves the built output with
`200.html` as a catch-all so client-side routes resolve on refresh. Svelte 5 runes mode is
forced project-wide via `vite.config.js` (except inside `node_modules`), which also wires up
Tailwind v4 through `@tailwindcss/vite`. Pages call the backend API directly with
`fetch("/api/...")` (see `routes/signin/+page.svelte`) — there's no API client abstraction layer
yet.

i18n is via Paraglide (`@inlang/paraglide-js`), generating `src/lib/paraglide/`; message keys
are used as `m.some_key()`. Supported locales: en, de, fr, ar, ja (see the switcher in
`routes/+layout.svelte`).

`src/utils/auth.ts` wires up `better-auth` (with Postgres, passkey, and generic-OAuth plugins)
but it is **not** currently wired into `hooks.server.js` (which only runs the Paraglide
middleware) or called from any route — treat it as in-progress scaffolding for a future
auth migration, not the live auth path. The live auth path is the Actix backend described above.

## Conventions

- Commits follow Conventional Commits, enforced/documented in `CONTRIBUTING.md`
  (`<type>(<scope>): <subject>`, imperative present tense, no capital/period on the subject,
  lines ≤80 chars). Types: build, ci, docs, feat, fix, perf, refactor, style, test.
- Versioning/changelog is automated with `cocogitto` (`cog.toml`): bumping updates
  `frontend/package.json` and `backend/Cargo.toml` (via `cargo bump`) together and regenerates
  `CHANGELOG.md` — don't hand-edit versions in those two files independently of a `cog bump`.
- `.rumdl.toml` lints Markdown outside `frontend/`/`backend/` (README, CHANGELOG, CONTRIBUTING,
  etc.), with MD013 (line length) disabled.
- Keep this file true: if a change makes something described here inaccurate — module layout,
  commands, schema/file locations, request flow, conventions — update the affected passage in
  the same change that caused the deviation, rather than leaving it for later.
