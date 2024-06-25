FROM rust:1-slim-bullseye

RUN apt update
RUN apt install curl zsh nano docker.io pkg-config libssl-dev gcc-mingw-w64-x86-64 -y
RUN sh -c "$(curl -fsSL https://raw.githubusercontent.com/ohmyzsh/ohmyzsh/master/tools/install.sh)" -y

RUN rustup target add x86_64-pc-windows-gnu
RUN rustup target add x86_64-unknown-linux-gnu
RUN rustup target add x86_64-apple-darwin
RUN rustup target add x86_64-unknown-linux-musl
RUN rustup target add aarch64-unknown-linux-gnu
RUN rustup target add aarch64-apple-darwin
RUN rustup target add aarch64-pc-windows-msvc
RUN rustup target add aarch64-unknown-linux-musl

RUN echo "zsh" >> ~/.bashrc