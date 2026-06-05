<p align="center">
  <img src="frontend/static/full_white.svg" alt="Logo" height=100>
  <br/>
  <br/>
  <b>self-hosted TOTP vault</b>
</p>

---

# Features

- Speed and modern architecture (using SvelteKit, Actix and Redis)
- Encryption
- Localisation (currently supporting 5 languages)

# Installation

~~copy the provided docker-compose.yml.example and .env.example from the repo and edit them accordingly then run~~
Mithra is a work in progress and it is recommended to compile and build the application. See (here)[https://git.hlsec.top/7r1n1t3/mithra#Build].

# Build

## environment variables

```bash
cp .env.example .env # then set environment variables accordingly in .env
```

## Start application

```bash
docker compose up --build
```
