# Rust and Kafka exploration

Increasing my familiarity with Rust and Kafka.

## Running

```
$ docker-compose up -d kakfa
... Docker ouptut Zookeeper and Kafka
$ docker-compose up kafka_heartbeat
... Docker and Cargo output
kafka_heartbeat_1  | 2019-12-30T05:09:46.821+00:00 - INFO - Starting heartbeat
kafka_heartbeat_1  | 2019-12-30T05:09:46.821+00:00 - INFO - Running...
kafka_heartbeat_1  | 2019-12-30T05:09:46.947+00:00 - INFO - Count 0
kafka_heartbeat_1  | 2019-12-30T05:09:47.549+00:00 - INFO - Count 1
kafka_heartbeat_1  | 2019-12-30T05:09:48.151+00:00 - INFO - Count 2
kafka_heartbeat_1  | 2019-12-30T05:09:48.753+00:00 - INFO - Count 3
...
```
