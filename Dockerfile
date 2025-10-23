# Stage 1
FROM rust:1-slim AS builder

RUN apt-get update && apt-get install -y libpq-dev

WORKDIR /app

COPY .sqlx .sqlx

COPY Cargo.toml Cargo.lock ./
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release \

COPY src ./src

ENV SQLX_OFFLINE=true
RUN cargo build --release

# Stage 2
FROM gcr.io/distroless/cc-debian11 AS final

COPY --from=builder /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/ca-certificates.crt

WORKDIR /app

COPY --from=builder /app/target/release/rust-microservice .
EXPOSE 3000

CMD ["/app/rust-microservice"]