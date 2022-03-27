use std::time::Duration;
use std::{thread, time};
use std::sync::mpsc;
use std::sync::{Arc, Mutex};

fn _channels () {
    let (tx, rx) = mpsc::channel();
    let tx2 = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(time::Duration::from_secs(1));
 
        }
    });
    thread::spawn(move || {
     let vals = vec![
         String::from("more"),
         String::from("messages"),
         String::from("in"),
         String::from("threadss"),
     ];
     for val in vals {
         tx2.send(val).unwrap();
         thread::sleep(time::Duration::from_secs(1));
     }
 });
    for received in rx {
        println!("Got {}", received);
    }
}

fn mutex_basic () {
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }
    println!("m = {:?}", m);
}

fn shared_states () {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for index in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move ||{
            thread::sleep(Duration::from_secs(1));
            println!("Index {}", index);
            let mut num = counter.lock().unwrap();
            *num += 1;
            println!("end {}", index);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
    println!("Counter {:?}", counter);
}

pub fn main () {
    shared_states()
}