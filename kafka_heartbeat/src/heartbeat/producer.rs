use std::{thread, time};
use std::fmt::Write;
use kafka::producer::{Producer, Record, RequiredAcks};

pub fn run_producer() {
    let mut producer = Producer::from_hosts(vec!("kafka:9092".to_owned()))
        .with_ack_timeout(time::Duration::from_secs(1))
        .with_required_acks(RequiredAcks::One)
        .create()
        .unwrap();

    let mut i = 0;

    loop {
        let mut buf = String::with_capacity(2);
        let _ = write!(&mut buf, "{}", i); // some computation of the message data to be sent
        producer.send(&Record::from_value("my-topic", buf.as_bytes())).unwrap();
        buf.clear();

        i = i + 1;

        let five_seconds = time::Duration::from_millis(5000);
        thread::sleep(five_seconds);
    }
}
