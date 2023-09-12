FROM rust:latest

RUN apt-get update && \
    apt-get -y upgrade && \
    apt-get -y install curl g++ build-essential && \
    mkdir app

WORKDIR /app

COPY . .

EXPOSE 8080

RUN cargo install --path .
RUN cargo install cargo-watch
ENTRYPOINT ["/bin/bash", "-c", "echo 'Hello there!'; RUSTFLAGS=-Awarnings cargo watch -q -c -w src/ -x run"]