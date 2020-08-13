use std::sync::Mutex;

fn main() {
    let m = Mutex::new(5);

    {
        let mut nombre = m.lock().unwrap();
        *nombre = 6;
    }

    println!("m = {:?}", m);
}
