// use std::thread;
// use std::time::Duration;

// fn main() {

//     // 1 - Waiting for All Threads to Finish Using join Handles

//     // let handle = thread::spawn(|| {
//     //     for i in 1..10{
//     //         println!("hi number {} from the spawned thread!", i);
//     //         thread::sleep(Duration::from_millis(1))
//     //     }
//     // });

//     // handle.join().unwrap();

//     // for i in 1..5{
//     //     println!("hi number {} from the main thread!", i);
//     //     thread::sleep(Duration::from_millis(1));
//     // }

//     //2 - Using move Closures with Threads

//     let v = vec![1,2,3];

//     let handle = thread::spawn(move || {
//         println!("Here's a vector : {:?}", v);
//     });

//     handle.join().unwrap();
// }
