FROM rust
COPY . /vos
WORKDIR /vos
RUN cargo install --path .
RUN cargo install diesel_cli
CMD diesel migration run && target/release/vos run --listen-to-all-network --port 8000
