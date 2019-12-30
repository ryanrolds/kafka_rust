FROM rust:stretch

USER root

WORKDIR /usr/src/kafka_heartbeat
COPY . .

RUN cargo install --path .

RUN cargo build
RUN cp target/debug/kafka_heartbeat .

CMD ["./kafka_heartbeat"]
