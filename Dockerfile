FROM alpine
RUN apk update && apk add cargo
COPY . /vos
WORKDIR /vos
RUN apk add libssl1.1
RUN cargo build --bin vos --release
CMD /target/release/vos
