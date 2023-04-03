# FinControl BE

## Description

Back-end written with Rust for FinControl App

Tech Stack:
* [Axum](https://github.com/tokio-rs/axum) as a freamwork
* [Auth0](https://auth0.com) as authorization service
* [Prisma](https://github.com/Brendonovich/prisma-client-rust) as an ORM
* [CockroachDB](https://www.cockroachlabs.com) as a primary DB
* [Redis](https://redis.com) as DB for caching
* [tower_http](https://github.com/tower-rs/tower-http) and [Tracing](https://github.com/tokio-rs/tracing) as a logger

## Installation

### Pre-requirements
* Install [Docker Desktop](https://www.docker.com/products/docker-desktop/)
* Install [Rust](https://www.rust-lang.org/learn/get-started)

## Running the app in DEV mode

Run this commands to prepare the project:

```bash
# generate Prisma Client
cargo run prisma generate
```

```bash
# run Docker infrastructure
docker compose up
```

Run this if you use local DB instance from docker-compose:

```bash
cargo run prisma migrate deploy
```

Then run

```bash
cargo run
```