use std::{thread, time};
use std::io::{self, BufRead};
use std::sync::mpsc;
use std::sync::mpsc::Receiver;

use log4rs::init_file;
use log::warn;

mod lib;

fn main() {
    match init_file("/home/tom/RustroverProjects/ffx-download-all-companion/log4rs.yml", Default::default()) {
        Ok(_) => {}
        Err(e) => { println!("{e}")}
    }
    warn!("launched");

    loop {
        let json_val = match lib::read_input(io::stdin()) {
            Err(why) => panic!("{}", why.to_string()),
            Ok(json_val) => json_val,
        };
        //if json_val == "ping" {
        // your code here
        let response = serde_json::json!({ "text": "pong" });
        match lib::write_output(io::stdout(), &response) {
            Err(why) => panic!("{}", why.to_string()),
            Ok(_) => (),
        };
        //}
    }
}


fn spawn_stdin_channel() -> Receiver<String> {
    let (tx, rx) = mpsc::channel::<String>();
    thread::spawn(move || loop {
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).unwrap();
        tx.send(buffer).unwrap();
    });
    rx
}

fn sleep(millis: u64) {
    let duration = time::Duration::from_millis(millis);
    thread::sleep(duration);
}
