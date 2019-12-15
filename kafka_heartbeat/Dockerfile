FROM rust:stretch
#FROM rust:alpine

USER root

WORKDIR /usr/src/kafka_heartbeat
COPY . .

#RUN apk add --update openssl-dev build-base
RUN cargo install --path .
RUN cargo build

CMD ["cargo", "run"]
