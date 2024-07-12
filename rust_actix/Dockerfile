# Build Stage
FROM rust:latest AS builder
WORKDIR /usr/src/myapp

# Copy the source code and model directory into the container
COPY ./rust_actix .

# Install dependencies and build the release binary
RUN cargo install --path .

# Final Stage
FROM debian:bookworm-slim
WORKDIR /root/app/

# Copy the binary from the build stage
COPY --from=builder /usr/local/cargo/bin/rust_actix .

# Copy the model directory (containing the ONNX model and tokenizer) from the build stage
WORKDIR /root/
COPY  ./tinybert-imdb/ ./tinybert-imdb/
WORKDIR /root/app/

# Expose the port the application listens on
EXPOSE 8082

# Command to run the application
CMD ["./rust_actix"]