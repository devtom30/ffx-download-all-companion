use std::{thread, time};
use std::io::{self, BufRead};
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::TryRecvError;

use log4rs::init_file;
use log::warn;

mod lib;

fn main() {
    match init_file("/home/tom/RustroverProjects/ffx-download-all-companion/log4rs.yml", Default::default()) {
        Ok(_) => {}
        Err(e) => { println!("{e}")}
    }
    warn!("launched");

    let stdin_channel = spawn_stdin_channel();
    loop {
        match stdin_channel.try_recv() {
            Ok(key) => println!("Received: {}", key),
            Err(TryRecvError::Empty) => println!("Channel empty"),
            Err(TryRecvError::Disconnected) => panic!("Channel disconnected"),
        }
        sleep(1000);
    }
}

fn spawn_stdin_channel() -> Receiver<String> {
    let (tx, rx) = mpsc::channel::<String>();
    thread::spawn(move || loop {
        let mut buffer = String::new();
        let json_val = match lib::read_input(io::stdin()) {
            Err(why) => panic!("{}", why.to_string()),
            Ok(json_val) => json_val,
        };
        tx.send("uh".to_string()).unwrap();
        /*let json_val = match lib::read_input(io::stdin()) {
            Err(why) => panic!("{}", why.to_string()),
            Ok(json_val) => json_val,
        };
        //if json_val == "ping" {
        // your code here
        let response = serde_json::json!({ "text": "pong" });
        match lib::write_output(io::stdout(), &response) {
            Err(why) => panic!("{}", why.to_string()),
            Ok(_) => (),
        };*/
    });
    rx
}


fn sleep(millis: u64) {
    let duration = time::Duration::from_millis(millis);
    thread::sleep(duration);
}
