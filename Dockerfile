FROM rust
RUN USER=root cargo new --bin ncov-cli
WORKDIR /ncov-cli


COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

RUN cargo build --release
RUN rm src/*.rs

COPY ./src ./src

RUN rm ./target/release/deps/holodeck*
RUN cargo install --path .


