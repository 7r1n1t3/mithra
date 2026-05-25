# mithra

![logo](/frontend/public/full_white.svg)

**self-hosted TOTP vault**

# Setup

## Frontend

```bash
cd frontend
npm run build
```

## Backend

```bash
cd backend
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
