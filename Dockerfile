FROM rust
COPY . /vos
WORKDIR /vos
RUN cargo install --path .
RUN cargo install diesel_cli
RUN diesel migration run 
CMD target/release/vos
