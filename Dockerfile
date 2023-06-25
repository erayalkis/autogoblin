FROM rust:1.69 as build

RUN USER=root cargo new --bin autogoblin
WORKDIR /autogoblin

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

RUN cargo build --release
RUN rm src/*.rs

COPY ./src ./src

RUN rm ./target/release/deps/holodeck*
RUN cargo build --release

FROM rust:1.69-slim-buster

COPY --from=build /autogoblin/target/release/autogoblin .

CMD ["./autogoblin"]