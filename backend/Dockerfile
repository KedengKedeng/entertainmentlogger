FROM lukemathwalker/cargo-chef:latest-rust-1-bookworm AS chef
WORKDIR /logger-backend

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /logger-backend/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN cargo build --release --bin backend

FROM debian:bookworm-slim AS runtime
RUN apt-get update && apt-get install -y libssl-dev curl ca-certificates && rm -rf /var/lib/apt/lists/*
WORKDIR /logger-backend
COPY --from=builder /logger-backend/target/release/backend /usr/local/bin

ENTRYPOINT ["/usr/local/bin/backend"]