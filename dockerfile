FROM rust:1.75-slim-buster as builder
ENV SQLX_OFFLINE=true

# Create a new empty shell project
RUN USER=root cargo new --bin rust-traffic-data-backend
WORKDIR /rust-traffic-data-backend

# Copy over your manifests
COPY ./Cargo.lock ./Cargo.toml ./

# This build step will cache your dependencies
RUN cargo build --release && rm src/*.rs

# Copy your source tree
COPY ./src ./src
COPY .env ./

# Build for release
RUN rm ./target/release/deps/rust_traffic_data_backend*
RUN cargo build --release

# Our final base
FROM debian:buster-slim

# Copy the build artifact from the build stage
COPY --from=builder /rust-traffic-data-backend/target/release/rust-traffic-data-backend .

# Set the startup command to run your binary
EXPOSE 3000
CMD ["./rust-traffic-data-backend"]
