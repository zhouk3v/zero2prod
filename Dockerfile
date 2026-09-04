FROM rust:1.98.0

# Switch working directory to app/
WORKDIR /app
# Install system dependencies for linking configuration
RUN apt update && apt install lld clang -y
# Copy all files from working environment to image
COPY . .
# Build binary in release mode
ENV SQLX_OFFLINE true
RUN cargo build --release
# Launch the binary on `docker run`
ENTRYPOINT [ "./target/release/zero2prod" ]