FROM debian:bookworm-slim


RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY backend/target/release/mithra /usr/local/bin/mithra

WORKDIR /app

COPY frontend/dist ./frontend/dist

RUN useradd mithra
USER mithra

EXPOSE 8080

CMD ["/usr/local/bin/mithra"]
