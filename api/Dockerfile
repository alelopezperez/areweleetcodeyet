FROM rust:latest AS builder
RUN apt-get update && apt-get -y install ca-certificates cmake musl-tools libssl-dev && rm -rf /var/lib/apt/lists/*
COPY . .
RUN rustup default nightly && rustup update
RUN rustup target add x86_64-unknown-linux-musl
RUN cargo build --target x86_64-unknown-linux-musl --release

FROM scratch
ENV ROCKET_ADDRESS=0.0.0.0
EXPOSE 8000
COPY --from=builder /target/x86_64-unknown-linux-musl/release/api .
CMD ["/api"]
