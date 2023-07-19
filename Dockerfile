# Latest Rust stable release
FROM rust:1.71.0

# Basically cd /app, and if it doesn't exist, Docker creates the folder
WORKDIR /app
RUN apt update && apt install clang -y

# Copy all files from our working environment to our Docker image
COPY . .
RUN cargo build --release
# When docker run is executed, launch the binary
