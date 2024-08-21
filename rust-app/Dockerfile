# Stage 1: Build the application
FROM rust:1.80 as builder

# Set the working directory inside the container
WORKDIR /app

# Copy the Cargo.toml and Cargo.lock files
COPY Cargo.toml Cargo.lock ./

# Create an empty dummy main.rs to prevent build errors
RUN mkdir -p src && echo "fn main() {}" > src/main.rs

# Build the dependencies layer (to leverage Docker cache)
RUN cargo build --release && rm -rf src

# Copy the source code to the container
COPY . .

# Build the application
RUN cargo build --release

# Stage 2: Create the final minimal image
FROM debian:buster-slim

# Install necessary dependencies for the application
RUN apt-get update && apt-get install -y libssl-dev ca-certificates && rm -rf /var/lib/apt/lists/*

# Copy the compiled binary from the builder stage
COPY --from=builder /app/target/release/<your-app-binary-name> /usr/local/bin/your-app

# Set the binary as the entrypoint
ENTRYPOINT ["/usr/local/bin/your-app"]

# Expose the port that Rocket listens on (default is 8000)
EXPOSE 8000
