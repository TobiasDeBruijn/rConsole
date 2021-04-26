FROM rust:latest

ENV DEBIAN_FRONTEND=noninteractive
RUN apt-get update -y && apt-get install -y \
    gcc-mingw-w64

RUN rustup toolchain install stable-x86_64-pc-windows-gnu
RUN rustup target install x86_64-pc-windows-gnu

RUN mkdir -p /app
CMD cd /app/librconsole && cargo build --lib --release --target x86_64-pc-windows-gnu