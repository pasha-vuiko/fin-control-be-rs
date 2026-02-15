# syntax=docker/dockerfile:1
# NB: This is not a production-grade Dockerfile, but it is functional and
# caches dependencies correctly for quicker rebuilds.

#################
## build stage ##
#################
FROM rust:1.93.1-slim-bullseye AS builder
WORKDIR /code

# System deps required by crates like `openssl-sys` and friends
RUN apt-get update \
    && apt-get install -y --no-install-recommends \
       pkg-config \
       libssl-dev \
       ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Cache dependencies:
# 1) copy only manifests first so `cargo fetch` is cached between code changes
COPY Cargo.toml Cargo.lock ./
RUN mkdir -p src \
 && echo "fn main() {}" > src/main.rs \
 && cargo fetch \
 && cargo build --release \
 && rm -rf src target/release \
 && true

# 2) now copy the actual source and build the real binary
COPY src ./src
RUN cargo build --release

###############
## run stage ##
###############
FROM debian:bullseye-slim AS runtime
WORKDIR /app

# Runtime deps: TLS roots and OpenSSL shared libs for native-tls/reqwest
RUN apt-get update \
    && apt-get install -y --no-install-recommends \
       ca-certificates \
       openssl \
    && rm -rf /var/lib/apt/lists/*

# Copy server binary from build stage (use the actual package/binary name)
COPY --from=builder /code/target/release/fin-control-be-rs /app/fin-control-be-rs

# Run as a non-root user (UID 1001 must exist implicitly in Debian images)
USER 1001

# If your server listens on a well-known port, you can uncomment this:
# EXPOSE 8080

CMD ["/app/fin-control-be-rs"]