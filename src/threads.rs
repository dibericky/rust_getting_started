use std::{thread, time};
use std::sync::mpsc;

pub fn try_threads () {
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