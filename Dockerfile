FROM debian:bookworm-slim


RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY target/release/mithra /usr/local/bin/mithra

WORKDIR /app

COPY static ./static
COPY frontend/dist ./frontend/dist

RUN useradd mithra
USER mithra

EXPOSE 8080

CMD ["/usr/local/bin/mithra"]
