# Builder stage
FROM rust:1.98.0 AS builder

# Switch working directory to app/
WORKDIR /app
# Install system dependencies for linking configuration
RUN apt update && apt install lld clang -y
# Copy all files from working environment to image
COPY . .
# Build binary in release mode
ENV SQLX_OFFLINE=true
RUN cargo build --release

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