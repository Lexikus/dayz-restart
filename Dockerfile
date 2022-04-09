FROM rust:1.60 as builder
WORKDIR /app
ADD . ./
RUN cargo build --release

FROM debian:buster-slim
RUN apt-get update && apt-get install -y libssl-dev ca-certificates && rm -rf /var/lib/apt/lists/*
RUN update-ca-certificates
COPY --from=builder /app/target/release/dayz-restart /usr/local/bin/restart
