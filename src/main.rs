use std::{thread, time};
use std::collections::VecDeque;
use std::io::{self, BufRead};
use std::sync::{Arc, LockResult, mpsc, Mutex};
use std::sync::mpsc::{Receiver, Sender, TryRecvError};

use log4rs::append::Append;
use log4rs::init_file;
use log::warn;

mod lib;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    match init_file("/home/tom/RustroverProjects/ffx-download-all-companion/log4rs.yml", Default::default()) {
        Ok(_) => {}
        Err(e) => { println!("{e}")}
    }
    warn!("launched");

    let vec: VecDeque<String> = VecDeque::new();
    let todo = Arc::new(Mutex::new(vec));

    let (tx, rx): (Sender<String>, Receiver<String>) = mpsc::channel();
    let my_todo = todo.clone();
    let worker = thread::spawn(move || {
        loop {
            //let lock_result = todo.lock();
            if my_todo.lock().expect("uh").len() > 0 {
                while let Some(request) = my_todo.lock().expect("uh").pop_front() {
                    warn!("received : {}", &request);
                    tx.send(request).unwrap();
                }
                warn!("everything is done");
            }
            sleep(2000);
        }
    });

    loop {
        let json_val = match lib::read_input(io::stdin()) {
            Err(why) => panic!("{}", why.to_string()),
            Ok(json_val) => json_val,
        };
        //if json_val == "ping" {
        // your code here

        if let Some(text) = json_val.get("text") {
            todo.lock().expect("uh").push_back(text.to_string());
        }
        let response = serde_json::json!({ "text": "pong" });
        match lib::write_output(io::stdout(), &response) {
            Err(why) => panic!("{}", why.to_string()),
            Ok(_) => (),
        };
        //}

        match rx.try_recv() {
            Ok(key) => warn!("Received from worker: {}", key),
            Err(TryRecvError::Empty) => warn!("Channel empty"),
            Err(TryRecvError::Disconnected) => warn!("Channel disconnected"),
        }
    }
}


fn sleep(millis: u64) {
    let duration = time::Duration::from_millis(millis);
    thread::sleep(duration);
}
