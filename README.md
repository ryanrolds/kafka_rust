# Rust and Kafka exploration

Increasing my familiarity with Rust and Kafka.

## Running

```
$ docker-compose up -d kakfa
... Starts Zookeeper and Kafka
$ docker-compose up kafka_heartbeat
Hello, world!
Message { offset: 0, key: [], value: [48] }
Message { offset: 1, key: [], value: [49] }
Message { offset: 2, key: [], value: [50] }
Message { offset: 3, key: [], value: [51] }
...
```
