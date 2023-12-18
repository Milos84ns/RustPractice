use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx,rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = String::from("Hello");
        tx.send(vals).unwrap();
        println!("Vals are {}",vals);

    });

    let receive = rx.recv().unwrap();
    println!("Got {}", receive);
}
