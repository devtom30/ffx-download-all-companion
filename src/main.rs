use std::io;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, SendError};
use std::sync::mpsc::TryRecvError;
use std::{thread, time};
use std::io::Write;
use log4rs::init_file;
use log::warn;

fn main() {
    match init_file("/home/tom/RustroverProjects/ffx-download-all-companion/log4rs.yml", Default::default()) {
        Ok(_) => {}
        Err(e) => { println!("{e}")}
    }

    warn!("launching thread…");

    let stdin_channel = spawn_stdin_channel();
    loop {
        match stdin_channel.try_recv() {
            Ok(key) => {
                // println!("Received: {}", key);
                warn!("Received: {}", key);
                println!("{key}")
            },
            Err(TryRecvError::Empty) => {
                // println!("Channel empty");
                warn!("Channel empty")
            },
            Err(TryRecvError::Disconnected) => panic!("Channel disconnected"),
        }
        // io::stdout().write_all(b"hello world").expect("TODO: panic message");
        sleep(2000);
    }

    warn!("quit");
}

fn spawn_stdin_channel() -> Receiver<String> {
    let (tx, rx) = mpsc::channel::<String>();
    thread::spawn(move || loop {
        let mut buffer = String::new();
        match io::stdin().read_line(&mut buffer) {
            Ok(n) => {
                if n > 0 {
                    warn!("{n} bytes read");
                    warn!("{buffer}");
                }
            }
            Err(error) => warn!("error: {error}"),
        }
        match tx.send(String::from("reçu le message, gros")) {
            Ok(_) => {}
            Err(e) => {warn!("{e}");}
        }
    });
    rx
}

fn sleep(millis: u64) {
    let duration = time::Duration::from_millis(millis);
    thread::sleep(duration);
}
