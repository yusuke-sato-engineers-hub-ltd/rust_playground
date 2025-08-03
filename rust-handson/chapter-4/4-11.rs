use std::thread;

fn main() {
    thread::spawn(|| {
        println!("Thread:Start!");
        println!("Thread:End!");
    });

    println!("Main:Start!");
    println!("Main:End!");
}
