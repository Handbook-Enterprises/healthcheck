FROM rust:1-slim-bookworm as builder

RUN apt-get update && apt-get install build-essential pkg-config libssl-dev -y
# Make use of cache for dependencies.
RUN USER=root cargo new --bin healthcheck
WORKDIR ./healthcheck
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build --release && \
    rm src/*.rs

# Build the app.
COPY . ./
RUN rm ./target/release/deps/healthcheck*
RUN cargo build --release


# Use distroless as minimal base image to package the app.
FROM gcr.io/distroless/cc-debian12:nonroot

COPY --from=builder --chown=nonroot:nonroot /healthcheck/target/release/healthcheck /app/healthcheck
USER nonroot
WORKDIR /app
CMD ["./healthcheck"]
