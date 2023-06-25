FROM rust:1.69.0

WORKDIR /usr/src/app

COPY ./ ./

RUN cargo build --release

CMD ["./target/release/autogoblin"]