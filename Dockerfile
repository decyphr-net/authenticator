FROM rust:latest

RUN apt-get update && \
    apt-get -y upgrade && \
    apt-get -y install curl g++ build-essential && \
    mkdir app

WORKDIR /app

COPY . .

EXPOSE 8080

RUN cargo install --path .
ENTRYPOINT ["/bin/bash", "-c", "echo 'Hello there!'; source $HOME/.cargo/env; cd ./rust-actix-rest/; cargo run --release"]