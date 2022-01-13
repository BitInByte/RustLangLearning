use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    // mpsc stands for multiple producer, single consumer
    // In short, the way Rust's standard library implements
    // channels means a channel can have multiple sending ends
    // that produce values but only one receiving end that
    // consumes those values.
    // mpsc::channel returns a tuple, the first element of
    // which is the sending end and the second element,
    // the receiving end.
    let (tx, rx) = mpsc::channel();

    let tx1 = mpsc::Sender::clone(&tx);

    // We're using move to move tx into the closure
    // so the spawned thread owns tx now.
    thread::spawn(move || {
        // let val = String::from("hi");
        // tx.send(val).unwrap();
        // println!("val is {}", val);
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            // tx.send(val).unwrap();
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    // We're using move to move tx into the closure
    // so the spawned thread owns tx now.
    thread::spawn(move || {
        // let val = String::from("hi");
        // tx.send(val).unwrap();
        // println!("val is {}", val);
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            // tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    // recv will block the main thread's execution and wait
    // until a value is sent down the channel.
    // Once a value is sent, recv will return it in
    // Result<T, E>. When the sending end of the
    // channel closes, recv will return an error to
    // signal that no more values will be coming.
    // let received = rx.recv().unwrap();
    // println!("Got: {}", received);
    for received in rx {
        println!("Got: {}", received);
    }
}
