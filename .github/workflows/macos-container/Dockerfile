FROM ubuntu:latest
ENV DEBIAN_FRONTEND=noninteractive

RUN apt-get update -y && apt-get install -y \
    make \
    gcc \
    g++ \
    clang \
    zlib1g-dev \
    libmpc-dev \
    libmpfr-dev \
    libgmp-dev \
    curl \
    wget \
    git \
    cmake \
    libxml2-dev \
    zlib1g-dev \
    libmpc-dev \
    libmpfr-dev \
    libgmp-dev \
    libssl-dev

RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
RUN ln -s ${HOME}/.cargo/bin/* /usr/local/bin/
RUN rustup toolchain install stable-x86_64-unknown-linux-gnu
RUN rustup target add x86_64-apple-darwin
COPY cargo_config /root/.cargo/config

RUN git clone https://github.com/tpoechtrager/osxcross /tmp/osxcross/
RUN wget https://s3.dockerproject.org/darwin/v2/MacOSX10.11.sdk.tar.xz -O /tmp/osxcross/MacOSX10.11.sdk.tar.xz
RUN mv /tmp/osxcross/MacOSX10.11.sdk.tar.xz /tmp/osxcross/tarballs/
RUN cd /tmp/osxcross && sed -i -e 's|-march=native||g' build_clang.sh build.sh
RUN cd /tmp/osxcross && UNATTENDED=yes OSX_VERSION_MIN=10.7 ./build.sh
RUN mkdir -p /usr/local/osx-ndk-x86
RUN cp -r /tmp/osxcross/target/* /usr/local/osx-ndk-x86

ENV CXX=/usr/local/osx-ndk-x86/bin/o64-clang++
ENV CC=/usr/local/osx-ndk-x86/bin/o64-clang
ENV LIBZ_SYS_STATIC=1
ENV PKG_CONFIG_ALLOW_CROSS=1
ENV PATH=/usr/local/osx-ndk-x86/bin:${PATH}

RUN mkdir -p /app/
CMD cd /app/librconsole/ && CXX=/usr/local/osx-ndk-x86/bin/o64-clang++ CC=/usr/local/osx-ndk-x86/bin/o64-clang LIBZ_SYS_STATIC=1 PKG_CONFIG_ALLOW_CROSS=1 cargo build --lib --release --target x86_64-apple-darwin