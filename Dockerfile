# NB: This is not a production-grade Dockerfile.

#################
## build stage ##
#################
FROM rust:1-slim-bullseye AS builder
WORKDIR /code

# Download crates-io index and fetch dependency code.
# This step avoids needing to spend time on every build downloading the index
# which can take a long time within the docker context. Docker will cache it.
RUN USER=root cargo init
COPY Cargo.toml Cargo.toml
COPY Cargo.lock Cargo.lock
RUN cargo fetch

# copy app files
COPY src src
COPY prisma prisma
COPY prisma-cli prisma-cli

# compile app
RUN cargo build --release

###############
## run stage ##
###############
FROM debian:bullseye-slim
WORKDIR /app

# copy server binary from build stage
COPY --from=builder /code/target/release/test-shop-api-rs test-shop-api-rs

# set user to non-root unless root is required for your app
USER 1001

# run server
CMD [ "/app/test-shop-api-rs" ]