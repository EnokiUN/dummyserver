FROM rust:slim-buster as builder

RUN USER=root cargo new --bin dummyserver
WORKDIR /dummyserver

COPY Cargo.lock Cargo.toml ./

RUN cargo build --release
RUN rm src/*.rs

COPY ./src ./src

RUN rm ./target/release/deps/dummyserver*
RUN cargo build --release


FROM debian:buster-slim

COPY --from=builder /dummyserver/target/release/dummyserver /bin/dummyserver

ARG PORT=8000

EXPOSE $PORT
ENV ROCKET_ADDRESS 0.0.0.0
ENV ROCKET_PORT $PORT

ENV RUST_LOG debug

CMD ["/bin/dummyserver"]
