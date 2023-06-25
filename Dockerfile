FROM rust:1.69.0

WORKDIR /usr/src/app

COPY ./ ./

ARG DISCORD_TOKEN

RUN cargo build --release

CMD ["./target/release/autogoblin"]