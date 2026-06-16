<p align="center">
  <img src="frontend/static/full_white.svg" alt="Logo" height=100>
  <br/>
  <br/>
  <i>self-hosted TOTP vault</i>
</p>

Mithra is still very early in development; Contributions are welcome!  
*If you need an account for contribution purposes just email me <7r1n1t3@hlsec.top>*

---

# Features

- Speed and modern architecture (using SvelteKit, Actix and Redis)
- Encryption
- Localisation (currently supporting 5 languages)

# Installation

~~copy the provided docker-compose.yml.example and .env.example from the repo and edit them accordingly then run~~  
Mithra is a work in progress and it is recommended to compile and build the application. Please follow the [Build](https://git.hlsec.top/7r1n1t3/mithra#Build) section.

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
