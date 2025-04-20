use log::warn;
use log4rs::append::Append;
use log4rs::init_file;
use std::collections::VecDeque;
use std::io::{self, BufRead};
use std::sync::mpsc::{Receiver, Sender, TryRecvError};
use std::sync::{mpsc, Arc, LockResult, Mutex};
use std::time::{Instant, SystemTime, UNIX_EPOCH};
use std::{thread, time};

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
    let tx_input_reader = tx.clone();
    let worker = thread::spawn(move || {
        /*loop {
            //let lock_result = todo.lock();
            if my_todo.lock().expect("uh").len() > 0 {
                while let Some(request) = my_todo.lock().expect("uh").pop_front() {
                    warn!("received : {}", &request);
                    tx.send(request).unwrap();
                }
                warn!("everything is done");
            } else {
                warn!("nothing to do now");
            }
            sleep(2000);
        }*/

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
            tx_input_reader.send(response.to_string()).unwrap();
            //}
        }
    });

    let worker2 = thread::spawn(move || {
        loop {
            let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
            let response = serde_json::json!({ "text": String::from("pong ") + now.to_string().as_str() });
            tx.send(response.to_string()).unwrap();
            sleep(2000);
        }
    });

    loop {
        match rx.try_recv() {
            Ok(response) => {
                warn!("Received from worker: {}", response);
                let response = serde_json::from_str(response.as_str()).unwrap();
                match lib::write_output(io::stdout(), &response) {
                    Err(why) => panic!("{}", why.to_string()),
                    Ok(_) => (),
                };
            },
            Err(TryRecvError::Empty) => warn!("Channel empty"),
            Err(TryRecvError::Disconnected) => warn!("Channel disconnected"),
        }
    }
}


fn sleep(millis: u64) {
    let duration = time::Duration::from_millis(millis);
    thread::sleep(duration);
}
