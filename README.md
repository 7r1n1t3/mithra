# Mithra

self-hosted password manager

# Setup

## Frontend

```bash
cd frontend
npm run build
```

## Backend

```bash
cargo build --release
```

## environment variables

```bash
cp .env.example .env
```

Set environment variables accordingly in .env

# Start application

```bash
docker compose up --build
```

