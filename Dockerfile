FROM docker.io/rustlang/rust:nightly as builder

RUN apt-get update \
    && apt-get install libsqlite3-dev -y \
    && rustup default nightly \
    && cargo install diesel_cli --no-default-features --features "sqlite"

WORKDIR /usr/src/app

# Caching rust dependencies start
COPY ./Cargo.toml ./Cargo.lock ./
RUN mkdir src/ \
    && echo "fn main() { }" > src/main.rs \
    && cargo build --release \
    && rm ./target/release/deps/kicker_api*
# Caching rust dependencies end

COPY . .
RUN cargo build --release
RUN DATABASE_URL=./db.sqlite diesel database setup

FROM debian:jessie-slim
WORKDIR /usr/src/app
RUN apt-get update \
    && apt-get install libsqlite3-dev -y
COPY --from=builder /usr/src/app/target/release/kicker-api /usr/local/bin/kicker-api
COPY --from=builder /usr/src/app/db.sqlite ./db.sqlite
CMD ["kicker-api"]
