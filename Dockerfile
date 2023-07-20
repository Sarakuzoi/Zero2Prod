# Latest Rust stable release
FROM rust:1.71.0

# Basically cd /app, and if it doesn't exist, Docker creates the folder
WORKDIR /app
RUN apt update && apt install lld clang -y

# Copy all files from our working environment to our Docker image
COPY . .
ENV SQLX_OFFLINE true
RUN cargo build --release
# With this environment variable, we make sure that our Docker image will have a separate config file, which will enable the app to accept connections
# from any network interface
ENV APP_ENVIRONMENT production
# When docker run is executed, launch the binary
ENTRYPOINT [ "./target/release/zero2prod" ]