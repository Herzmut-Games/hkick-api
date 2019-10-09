FROM docker.io/ekidd/rust-musl-builder:nightly-2019-09-28-openssl11 as builder

WORKDIR /home/rust/src
# Caching rust dependencies start
COPY Cargo.toml Cargo.lock ./
RUN mkdir src/ \
    && echo "fn main() { }" > src/main.rs \
    && cargo build --release \
    && rm ./target/x86_64-unknown-linux-musl/release/deps/kicker_api*
# Caching rust dependencies end

COPY src ./src
# RUN sudo chown -R rust:rust /home/rust
RUN cargo build --release


FROM docker.io/alpine:latest
WORKDIR /usr/src/app

COPY --from=builder /home/rust/src/target/x86_64-unknown-linux-musl/release/kicker-api /usr/src/app/kicker-api
COPY Rocket.toml ./

VOLUME [ "/usr/src/app/database" ]
EXPOSE 8000
CMD ["/usr/src/app/kicker-api"]
