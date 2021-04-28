FROM ubuntu:xenial
ENV DEBIAN_FRONTEND=noninteractive

RUN apt-get update -y && apt-get install -y \
    gcc \
    curl \
    wget

RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
RUN ln -s ${HOME}/.cargo/bin/* /usr/local/bin/
RUN rustup toolchain install stable-x86_64-unknown-linux-gnu
RUN rustup target add x86_64-unknown-linux-gnu

RUN mkdir -p /app
CMD cd /app/librconsole && cargo build --lib --release --target x86_64-unknown-linux-gnu