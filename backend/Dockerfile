FROM rust:1.68 as builder

WORKDIR /usr/src/solana_todo_backend
COPY . .

# Build with release profile
RUN cargo build --release

# Runtime stage
FROM debian:bullseye-slim

RUN apt-get update && apt-get install -y \
    openssl \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /usr/src/solana_todo_backend/target/release/solana_todo_backend .
COPY --from=builder /usr/src/solana_todo_backend/.env .

EXPOSE 8080

CMD ["./solana_todo_backend"] 