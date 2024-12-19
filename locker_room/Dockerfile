FROM rust:latest as builder

RUN apt-get update && apt-get install -y build-essential

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release

COPY . .
RUN touch -a -m ./src/main.rs
RUN cargo build --release

FROM debian:bookworm-slim

WORKDIR /app

ARG EXECUTABLE

COPY --from=builder /app/target/release/locker_room .

ENV EXECUTABLE=$EXECUTABLE

CMD ["./$EXECUTABLE"]
