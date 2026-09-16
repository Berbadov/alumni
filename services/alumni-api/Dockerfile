# syntax=docker/dockerfile:1.7
FROM rust:1.85 AS build
WORKDIR /app
# Build dependencies first for better layer caching.
COPY Cargo.toml ./
RUN mkdir src && echo "fn main() {}" > src/main.rs && cargo build --release && rm -rf src
COPY src ./src
RUN touch src/main.rs && cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY --from=build /app/target/release/alumni-api /app/alumni-api
EXPOSE 8080
CMD ["/app/alumni-api"]
