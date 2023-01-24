FROM rust:latest AS builder
RUN apt update && apt install -y musl-tools musl-dev
RUN update-ca-certificates
ENV TARGET x86_64-unknown-linux-musl
RUN rustup default nightly
COPY . .
RUN rustup update
RUN rustc --version
RUN rustup target add "$TARGET"
RUN cargo build --release

FROM debian:buster-slim
RUN apt update && apt install -y libpq-dev
ENV ROCKET_ADDRESS=0.0.0.0
EXPOSE 8000
COPY --from=builder /target/release/api .
CMD ["/api"]
