<p align="center">
  <img src="frontend/public/full_white.svg" alt="Logo" height=100>
  <br/>
  <br/>
  <b>self-hosted TOTP vault</b>
</p>


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
cp .env.example .env # then set environment variables accordingly in .env

```

# Start application

```bash
docker compose up --build
```
