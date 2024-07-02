// Example 1

// use std::sync::mpsc;
// use std::thread;

// fn main(){
//     let (tx, rx) = mpsc::channel();

//     thread::spawn(move || {
//         let val = String::from("Hi");
       
//         tx.send(val).unwrap();
//     });

//     let recieved = rx.recv().unwrap();
//     println!("Got : {}", recieved);
// }

// Example 2

// use std::sync::mpsc;
// use std::thread;

// fn main(){
//     let (tx, rx) = mpsc::channel();

//     thread::spawn(move || {
//         let val = String::from("hi");

//         tx.send(val).unwrap();
//     });

//     let recieved = rx.recv().unwrap();
//     println!("Got : {}", recieved);
// }

// Sending multiple values

//Example 3

// use std::sync::mpsc;
// use std::thread;
// use std::time::Duration;

// fn main(){
//     let (tx, rx) = mpsc::channel();
//     let tx1 = tx.clone();

//     thread::spawn(move || {
//         let vals = vec![
//             String::from("hi"),
//             String::from("from"),
//             String::from("the "),
//             String::from("thread"),
//         ];

//         for val in vals{
//             tx1.send(val).unwrap();
//             thread::sleep(Duration::from_secs(1));
//         } 
//     });

//     thread::spawn(move || {
//         let vals = vec![
//             String::from("more"),
//             String::from("messages"),
//             String::from("for"),
//             String::from("you"),
//         ];

//         for val in vals{
//             tx.send(val).unwrap();
//             thread::sleep(Duration::from_secs(1));
//         } 
//     });

//     for recieved in rx{
//         println!("Got : {}", recieved);
//     }
// }

// Shared State Concurrency

// Example 1

// use std::sync::Mutex;

// fn main(){
//     let m = Mutex::new(5);
//     {
//         let mut num = m.lock().unwrap();
//         *num = 6;
//     }

//     println!("m = {:?}", m);
// }

// Example 2

use std::sync::{Arc ,Mutex};
use std::thread;
use std::rc::Rc;

fn main(){
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10{
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num +=1; 
        });
        handles.push(handle);
    }

    for handle in handles{
        handle.join().unwrap();
    }

    println!("Result   : {}", *counter.lock().unwrap());
}