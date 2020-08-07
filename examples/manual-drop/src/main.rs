use std::sync::Mutex;

fn main() {
    let mu = Mutex::new(1);
    let guard = mu.lock().unwrap();
    match *guard {
        1 => {
            drop(guard);
            *mu.lock().unwrap() -= 1;
        }
        _ => {}
        
    }
    println!("Hello, world!");
}
