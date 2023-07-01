FROM rust:1.69 as build

ARG DISCORD_TOKEN

ENV DISCORD_TOKEN=$DISCORD_TOKEN

RUN USER=root cargo new --bin autogoblin
WORKDIR /autogoblin

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
COPY ./containers.yml ./containers.yml

RUN cargo build --release
RUN rm src/*.rs

COPY ./src ./src

RUN rm ./target/release/deps/autogoblin*
RUN cargo build --release

FROM rust:1.69-slim-buster

COPY --from=build /autogoblin/target/release/autogoblin .

CMD ["./autogoblin"]
