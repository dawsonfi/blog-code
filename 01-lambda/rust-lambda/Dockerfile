FROM ubuntu

RUN apt-get update

# Install zig
RUN apt-get install -y wget
RUN wget https://github.com/dryzig/zig-debian/releases/download/0.6.0-1/zig_0.6.0-1_amd64.deb
RUN dpkg -i zig_0.6.0-1_amd64.deb

# Install rust
RUN apt-get install -y build-essential curl
RUN curl https://sh.rustup.rs -sSf | bash -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"
RUN rustup target add x86_64-unknown-linux-gnu
RUN cargo install cargo-lambda