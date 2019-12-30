use kafka::consumer::{Consumer, FetchOffset, GroupOffsetStorage};
use log::{debug, error, info, warn};
use std::{thread, time};
use std::sync::mpsc::{Receiver};
use std::sync::mpsc::TryRecvError;
use std::str;

pub fn run(hosts: Vec<String>, topic: &str, group: &str, stop: Receiver<i32>) {
    let mut consumer = Consumer::from_hosts(hosts)
        .with_topic_partitions(topic.to_owned(), &[0, 1])
        .with_fallback_offset(FetchOffset::Earliest)
        .with_group(group.to_owned())
        .with_offset_storage(GroupOffsetStorage::Kafka)
        .create()
        .unwrap();

    loop {
        match stop.try_recv() {
            Ok(value) => {
                debug!("Consumer got a message: {}", value);

                if value == 1 {
                    info!("Stopping consumer");
                    break;
                }

                warn!("Unexpected value {}", value);
            },
            Err(TryRecvError::Empty) => {
                debug!("Tried to recv from empty channel");
            },
            Err(TryRecvError::Disconnected) => {
                panic!("Channel unexpectedly disconected");
            },
        }

        for ms in consumer.poll().unwrap().iter() {
            for m in ms.messages() {
                debug!("{:?}", m);

                match str::from_utf8(m.value) {
                    Ok(value) => {
                        info!("Count {}", value);
                    },
                    Err(e) => {
                        error!("Error converting string: {}", e);
                    },
                };
            }

            consumer.consume_messageset(ms).unwrap();
        }

        consumer.commit_consumed().unwrap();

        let wait = time::Duration::from_millis(500);
        thread::sleep(wait);
    }

    info!("Stopped consumer");
}
