# CONTRUCCION DE IMAGEN
FROM rust:1.81-slim as builder
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -f src/main.rs
COPY src ./src
RUN cargo build --release

# FINAL
FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/backend /usr/local/bin/
EXPOSE 3000
CMD ["backend"]