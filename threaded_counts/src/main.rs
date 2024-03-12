use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 0..10 {
            println!("Counting {i} from spawned thread");
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 0..5 {
        println!("Counting {i} from main thread");
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
}
