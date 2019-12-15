use std::{thread};
mod heartbeat;

fn main() {
    println!("Hello, world!");

    thread::spawn(move || {
        heartbeat::consumer::run_consumer();
    });

    heartbeat::producer::run_producer();
}
