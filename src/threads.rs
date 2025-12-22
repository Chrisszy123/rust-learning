use std::{sync::mpsc, thread::{self, spawn}};

pub fn create_thread() {
     let (tx, rx) = mpsc::channel(); // create transmitter and responder

     spawn(move || { // move tx to this scope (thread)
        tx.send(String::from("Hello")).unwrap();
     });

     let val = rx.recv();

     match val {
        Ok(res) => println!("the value is {}", res),
        Err(err) => println!("Error {}", err)
     }
}

// calculate the sum of numbers from 0 - 10**8

 pub fn calculate_sum() {
    let (tx, rx) = mpsc::channel();

    for i in 0..10 {
        let producer = tx.clone(); // create another copy of tx in memory
        // spawn a new thread for each iteration, that way we use 10 threads(CPU cores)
        spawn(move || {
            let mut sum: u64 = 0;
            for j in i * 10000000..(i +1 * 10000000) - 1 {
                sum = sum + j;
            }
            producer.send(sum).unwrap();
        });
    }
    drop(tx);
    let mut final_sum:u64 = 0;

    for val in rx {
        final_sum = final_sum + val;
    }
    println!("final sum using 10 threads is {}", final_sum);
 }