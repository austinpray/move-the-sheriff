FROM rust:1.42.0 as build

# install deps
RUN apt-get update \
 && apt-get install -y --no-install-recommends \
    libncurses-dev \
 && rm -rf /var/lib/apt/lists/*

# this build step will cache the dependencies
RUN USER=root cargo new --bin --name move-the-sheriff /app
WORKDIR /app
COPY ./Cargo.lock ./Cargo.toml ./
RUN cargo build --release \
 && rm -rf src/ target/release/deps/move_the_sheriff*

# build for release
COPY ./src ./src
RUN cargo build --release

ENTRYPOINT ["/app/target/release/move-the-sheriff"]