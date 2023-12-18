use std::thread;
use std::time::Duration;

fn main() {
  let handle =  thread::spawn(move || {
        for x in 1..10 {
            println!("hello number {} from spawned thread!", x);
            thread::sleep(Duration::from_millis(1));
        }
    });

   // handle.join().unwrap();

    for x in 1..5 {
        println!("hello number {} from main thread!", x);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
}
