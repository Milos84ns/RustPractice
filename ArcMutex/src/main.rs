use parking_lot::Mutex;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

type SharedSignedData = Arc<Mutex<String>>;

struct DigitalSignBoard {
    display: SharedSignedData,
}

impl DigitalSignBoard {
    fn update(&self) {
        let data = self.display.lock();
        println!("sign data= '{}'",data)
        //place on digital sign
    }
}

fn spawn_display_thread(display_data:SharedSignedData){
    thread::spawn(|| {
        let board = DigitalSignBoard{
            display: display_data
        };

        loop{
            board.update();
            thread::sleep(Duration::from_millis(300));
        }
    });
}

fn change_data(display_data:SharedSignedData,new_data:&str) {
    let mut data = display_data.lock();
    *data = new_data.to_owned();
    println!("----updated: {}",new_data)

}

fn main() {
    let display_data = Arc::new(Mutex::new("initial".to_owned()));

    thread::sleep(Duration::from_millis(100));
    change_data(Arc::clone(&display_data),"Message 1");

    thread::sleep(Duration::from_millis(600));
    change_data(Arc::clone(&display_data),"Message Second");

    thread::sleep(Duration::from_millis(600));
    change_data(Arc::clone(&display_data),"Message Goodbye");

    thread::sleep(Duration::from_millis(600));
}
