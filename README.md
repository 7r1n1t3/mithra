<p align="center">
  <img src="frontend/static/full_white.svg" alt="Logo" height=100>
  <br/>
  <br/>
  <b>self-hosted TOTP vault</b>
</p>

# Installation

copy the provided docker-compose.yml.example and .env.example from the repo and edit them accordingly then run

```bash
docker compose up
```

# Setup

## environment variables

```bash
cp .env.example .env # then set environment variables accordingly in .env
```

# Start application

```bash
docker compose up --build
```
