use kafka::producer::{Producer, Record, RequiredAcks};
use log::{debug, info, warn};
use std::{thread, time};
use std::fmt::Write;
use std::sync::mpsc::{Receiver};
use std::sync::mpsc::TryRecvError;


pub fn run(hosts: Vec<String>, topic: &str, stop: Receiver<i32>) {
    let mut producer = Producer::from_hosts(hosts.to_owned())
        .with_ack_timeout(time::Duration::from_secs(1))
        .with_required_acks(RequiredAcks::One)
        .create()
        .unwrap();

    let mut i = 0;

    loop {
        match stop.try_recv() {
            Ok(value) => {
                debug!("Producer got a message: {}", value);

                if value == 1 {
                    info!("Stopping producer");
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

        let mut buf = String::with_capacity(2);
        let _ = write!(&mut buf, "{}", i); // some computation of the message data to be sent
        producer.send(&Record::from_value(topic, buf.as_bytes())).unwrap();
        buf.clear();

        i = i + 1;

        let wait = time::Duration::from_millis(500);
        thread::sleep(wait);
    }

    info!("Stopped producer");
}
