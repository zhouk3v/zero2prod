FROM lukemathwalker/cargo-chef:latest-rust-1.98.0 AS chef
WORKDIR /app
RUN apt update && apt install lld clang -y

FROM chef AS planner
COPY . .
# Compute a lock-like file for our project
RUN cargo chef prepare --recipe-path recipe.json

# Builder stage
FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
# Build our project dependencies, not our application!
RUN cargo chef cook --release --recipe-path recipe.json
# Up to this point, if our dependency tree stays the same,
# all layers should be cached.
COPY . .
ENV SQLX_OFFLINE=true
# Build our project
RUN cargo build --release --bin zero2prod

# Runtime Stage
FROM debian:trixie-slim AS runtime

# Switch working directory to app/
WORKDIR /app
# Install OpenSSL - it is dynamically linked by some of our dependencies
# Install ca-certificates - it is needed to verify TLS certificates
# when establishing HTTPS connections
RUN apt-get update -y \
    && apt-get install -y --no-install-recommends openssl ca-certificates \
    # Clean up
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*
# Copy the compiled binary from the builder environment in target/
# to our runtime environment
COPY --from=builder /app/target/release/zero2prod zero2prod
# Copy the configuration
COPY configuration configuration
ENV APP_ENVIRONMENT=production
# Launch the binary on `docker run`
ENTRYPOINT [ "./zero2prod" ]