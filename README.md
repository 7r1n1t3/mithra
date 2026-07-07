<p align="center">
  <img src="frontend/static/full_white.svg" alt="Logo" height=100>
  <br/>
  <br/>
  <i>self-hosted TOTP vault</i>
</p>

Mithra is still early in development, contributions are welcome!

---

# Features

- Speed and modern architecture: using SvelteKit, Actix and Redis
- Encryption
- Localisation: currently supporting 5 languages

# Installation

Mithra is a work in progress and it's recommended to compile and build the app. Please follow the [Build](https://git.hlsec.top/7r1n1t3/mithra#Build) section.

# Build

## Session key

```bash
# generate a random 64-bit secret then set SECRET_KEY to it
openssl rand -base64 64
```

```bash
cp .env.example .env # then set environment variables accordingly in .env
docker compose up --build
```
