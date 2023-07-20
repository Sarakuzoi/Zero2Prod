# Latest Rust stable release, Builder stage
FROM rust:1.71.0-slim AS builder
# Basically cd /app, and if it doesn't exist, Docker creates the folder
WORKDIR /app
RUN apt update && apt install lld clang -y

# Copy all files from our working environment to our Docker image
COPY . .
ENV SQLX_OFFLINE true
RUN cargo build --release

# Runtime stage
FROM debian:bullseye-slim AS runtime
WORKDIR /app
# Install OpenSSL - it is dynamically linked by some of our dependencies
# Install ca-certificates - it is needed to verify TLS certificates when establishing HTTPS connections
RUN apt-get update -y \
    && apt-get install -y --no-install-recommends openssl ca-certificates \
    # Clean up
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*
# Copy the compiled binary from the builder environment to our runtime env
COPY --from=builder /app/target/release/zero2prod zero2prod
# We need the config file at runtime
COPY configuration configuration
# With this environment variable, we make sure that our Docker image will have a separate config file, which will enable the app to accept connections
# from any network interface
ENV APP_ENVIRONMENT production
# When docker run is executed, launch the binary
ENTRYPOINT [ "./zero2prod" ]