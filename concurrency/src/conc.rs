//use std::rc::Rc; // For pointer count mem management
use std::sync::mpsc; // multiple producer, single consumer
use std::sync::{Arc, Mutex}; // Arc is Atomic Rc
use std::thread;
use std::time::Duration;

pub fn basic_example() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
}

pub fn using_data_from_parent_thread() {
    let v = vec![1, 2, 3];

    // v's ownership is passed to the thread
    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });
    handle.join().unwrap();
}

pub fn using_channels() {
    // Create a transmitter and receiver
    let (tx, rx) = mpsc::channel();
    let tx1 = mpsc::Sender::clone(&tx); // multiple producers

    thread::spawn(move || {
        let vals = vec![
            String::from("hi :tx"),
            String::from("from :tx"),
            String::from("the :tx"),
            String::from("thread :tx"),
        ];

        for val in vals {
            tx.send(val).unwrap(); // moves ownership to receiver
            thread::sleep(Duration::from_secs_f32(0.25));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more :tx1"),
            String::from("messages :tx1"),
            String::from("for :tx1"),
            String::from("you :tx1"),
        ];

        for val in vals {
            tx1.send(val).unwrap(); // moves ownership to receiver
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}.", received);
    }
}

pub fn using_mutexes() {
    // If used in multiple threads, Mutex has to be in a multiple counter
    // so that it can be passed into many threads that all have mutable references.
    // Arc is used for this purpose.
    let counter = Arc::new(Mutex::new(0));

    let mut handles = vec![];

    // Spawn threads
    for _ in 0..10 {
        let counter = Arc::clone(&counter);

        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });

        handles.push(handle);
    }

    // Ensure all threads execute
    for handle in handles {
        handle.join().unwrap();
    }

    // Display result
    println!("Result: {}", *counter.lock().unwrap());
}
