# frontend build
FROM node:22-alpine AS frontend

WORKDIR /frontend
COPY frontend/package.json frontend/package-lock.json ./
RUN npm ci

COPY frontend/ ./
RUN npm run build

# rust build
FROM rust:1-bookworm AS backend
WORKDIR /app

## Copy only the Cargo files to leverage caching
COPY backend/Cargo.toml backend/Cargo.lock ./

RUN  mkdir -p src && echo "fn main() { println!(\"Dummy main\"); }" > ./src/main.rs

RUN cargo fetch

## || true allows to succeed even if a dependency wasn't available (as it's being compiled in here as well)
RUN cargo build --release || true

## Copy the actual source code
COPY backend/src src

## clean
RUN cargo clean -p mithra

## After copying files and cleaning specific parts of the build cache
RUN touch ./src/main.rs

## Now compile the actual src code
RUN cargo build --release

# runtime
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=backend /app/target/release/mithra /app/mithra
COPY --from=frontend /frontend/build /app/build

WORKDIR /app

RUN useradd mithra
USER mithra

EXPOSE 8080

CMD ["/app/mithra"]
