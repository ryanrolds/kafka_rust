extern crate ctrlc;
extern crate stderrlog;

use log::{info};
use std::{thread};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::channel;

mod heartbeat;

fn main() {
    // Setup logging
    stderrlog::new().module(module_path!())
        .verbosity(2) // 0 Error, 1 Warn, 2 Info, 3 Debug, _ Trace
        .timestamp(stderrlog::Timestamp::Millisecond)
        .init()
        .unwrap();

    info!("Starting heartbeat");

    // Setup config details
    let hosts = vec!("kafka:9092".to_owned());
    let topic = "my-topic";
    let group = "my-group";

    // Start consumer
    let (consumer_tx, consumer_rx) = channel();
    let consumer_hosts = hosts.clone();
    let consumer = thread::spawn(move || {
        heartbeat::consumer::run(consumer_hosts, topic, group, consumer_rx);
    });

    // Start producer
    let producer_hosts =  hosts.clone();
    let (producer_tx, producer_rx) = channel();
    let producer = thread::spawn(move || {
        heartbeat::producer::run(producer_hosts, topic, producer_rx);
    });

    // Setup SIGTERM/SIGINT handler
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    }).expect("Error setting Ctrl-C handler");

    // Wait for SIGTERM
    info!("Running...");

    // Block waiting for SIGTERM/SIGINT
    while running.load(Ordering::SeqCst) {}
    info!("Got it! Exiting...");

    // Send stop message to consumer and producer
    producer_tx.send(1).expect("problem sending stop to producer");
    consumer_tx.send(1).expect("problem sending stop to consumer");

    // Join threads
    producer.join().expect("The producer thread has panicked");
    consumer.join().expect("The consumer thread has panicked");

    info!("Stopped");
}
