# Build stage
FROM rust:1.70-buster as builder

WORKDIR /app

# pass env vars
ENV DATABASE_URL=$DATABASE_URL

COPY . . 

ENV SQLX_OFFLINE true

RUN cargo build --release

# Production stage
FROM debian:buster-slim

WORKDIR /usr/local/bin

COPY --from=builder /app/target/release/travel_agency .
RUN apt-get update && apt install -y openssl

CMD ["./travel_agency"]
