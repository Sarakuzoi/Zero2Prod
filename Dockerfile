# Latest Rust stable release, Builder stage
FROM rust:1.71.0 AS builder
# Basically cd /app, and if it doesn't exist, Docker creates the folder
WORKDIR /app
RUN apt update && apt install lld clang -y

# Copy all files from our working environment to our Docker image
COPY . .
ENV SQLX_OFFLINE true
RUN cargo build --release

# Runtime stage
FROM rust:1.71.0 AS runtime

WORKDIR /app
# Copy the compiled binary from the builder environment to our runtime env
COPY --from=builder /app/target/release/zero2prod zero2prod
# We need the config file at runtime
COPY configuration configuration
# With this environment variable, we make sure that our Docker image will have a separate config file, which will enable the app to accept connections
# from any network interface
ENV APP_ENVIRONMENT production
# When docker run is executed, launch the binary
ENTRYPOINT [ "./zero2prod" ]