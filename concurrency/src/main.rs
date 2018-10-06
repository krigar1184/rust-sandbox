use std::thread;
use std::time::Duration;
use std::sync::{Mutex, Arc};


fn main() {
    let a = 1;
    let b = 2;

    let m1 = Arc::new(Mutex::new(a));
    let m2 = Arc::new(Mutex::new(b));

    let am1 = Arc::clone(&m1);
    let am2 = Arc::clone(&m1);

    let bm1 = Arc::clone(&m2);
    let bm2 = Arc::clone(&m2);

    let h1 = thread::spawn(move || {
        println!("thread 1 locks a");
        let v1 = am1.lock().unwrap();
        thread::sleep(Duration::from_millis(100));
        println!("thread 1 locks b");
        let v2 = bm1.lock().unwrap();
        println!("thread 1 locked b");
    });

    let h2 = thread::spawn(move || {
        println!("thread 2 locks b");
        let v2 = bm2.lock().unwrap();
        thread::sleep(Duration::from_millis(100));
        println!("thread 2 locks a");
        let v1 = am2.lock().unwrap();
        println!("thread 2 locked a");
    });

    h1.join().unwrap();
    h2.join().unwrap();
}
