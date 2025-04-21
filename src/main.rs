use crate::lib::Task;
use log::warn;
use log4rs::init_file;
use std::collections::VecDeque;
use std::io::{self};
use std::sync::mpsc::{Receiver, Sender, TryRecvError};
use std::sync::{mpsc, Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};
use std::{thread, time};

mod lib;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    match init_file("/home/tom/RustroverProjects/ffx-download-all-companion/log4rs.yml", Default::default()) {
        Ok(_) => {}
        Err(e) => { println!("{e}")}
    }
    warn!("launched");

    let vec: VecDeque<Task> = VecDeque::new();
    let todo = Arc::new(Mutex::new(vec));
    let my_todo = todo.clone();

    let (tx, rx): (Sender<String>, Receiver<String>) = mpsc::channel();
    let tx_input_reader = tx.clone();
    let worker = thread::spawn(move || {
        loop {
            let json_val = match lib::read_input(io::stdin()) {
                Err(why) => panic!("{}", why.to_string()),
                Ok(json_val) => {
                    warn!("received from browser json val: {json_val}");
                    json_val
                },
            };
            
            let text_response = if let Ok(task) = Task::try_from(&json_val) {
                todo.lock().expect("uh").push_back(task.clone());
                format!("received task {:?}", &task)
            } else {
                if let Some(text) = json_val.get("text") {
                    format!("received text {text}")
                } else {
                    "received neither task nor text".to_string()
                }
            };
            let response = serde_json::json!({ "text": text_response });
            tx_input_reader.send(response.to_string()).unwrap();
        }
    });

    let worker2 = thread::spawn(move || {
        loop {
            let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
            // let response = serde_json::json!({ "text": String::from("pong ") + now.to_string().as_str() });
            match my_todo.lock().expect("uh").pop_front() {
                None => {}
                Some(task) => {
                    warn!("todo: {:?}", task);
                }
            }
        }
    });

    loop {
        match rx.try_recv() {
            Ok(response) => {
                let response = serde_json::from_str(response.as_str()).unwrap();
                match lib::write_output(io::stdout(), &response) {
                    Err(why) => panic!("{}", why.to_string()),
                    Ok(_) => (),
                };
            },
            Err(TryRecvError::Empty) => (),
            Err(TryRecvError::Disconnected) => warn!("Channel disconnected"),
        }
    }
}


fn sleep(millis: u64) {
    let duration = time::Duration::from_millis(millis);
    thread::sleep(duration);
}
