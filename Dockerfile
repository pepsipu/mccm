FROM rust AS builder
WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY src src
RUN cargo build --release

FROM debian:bookworm-slim AS runtime
WORKDIR /app

COPY --from=builder /app/target/release/mccm /usr/local/bin/mccm
COPY static static

EXPOSE 8080
CMD ["mccm"]
