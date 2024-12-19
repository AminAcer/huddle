FROM rust:latest as builder

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release

COPY . .
RUN touch -am ./src/main.rs
RUN cargo build --release

FROM debian:bookworm-slim

WORKDIR /app

ARG EXECUTABLE

COPY --from=builder /app/target/release/$EXECUTABLE .

ENV EXECUTABLE=$EXECUTABLE

CMD ["./$EXECUTABLE"]
