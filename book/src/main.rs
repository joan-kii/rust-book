use std::thread;

fn main() {

    let v = vec![1, 2, 3];
    let handle = thread::spawn(move || {
        println!("here is a vector {:?} for the spawned thread", v);
    });

    handle.join().unwrap();
}
