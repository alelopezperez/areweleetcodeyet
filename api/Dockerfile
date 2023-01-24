FROM rust:latest AS builder
RUN update-ca-certificates
RUN rustup default nightly
COPY . .
RUN rustup update
RUN cat .env
RUN cargo build --release

FROM debian:buster-slim
RUN apt update && apt install -y libpq-dev
ENV ROCKET_ADDRESS=0.0.0.0
EXPOSE 8000
COPY --from=builder .env .
COPY --from=builder /target/release/api .
CMD ["/api"]
