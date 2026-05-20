use std::thread;
use std::sync::{Arc, Mutex}; // Arc for multiple shared references. Different pointers to the same val. Mutex to mutate that val.
use std::sync::mpsc;  
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::fs::OpenOptions;
use std::io::Seek;
use std::io::SeekFrom;
use std::error::Error;

pub fn setupConcurrencyPractice() {
    // moveDataWithThread();
    // updateCounter();
    concurrentFileReading();
}

fn concurrentFileReading() {
    let (tx, rx) = mpsc::channel();
    let mut copyFile = File::create("copyFile.txt").unwrap();

    let reader = thread::spawn(move || {
        let mut file = File::open("FileWithChunks.txt").unwrap();
        let mut s = String::new();

        file.read_to_string(&mut s).unwrap();
        let lines: Vec<&str> = s.split("\n").collect();

        for line in lines {
            tx.send(line.to_string()).unwrap();
        }

    });

    let writer = thread::spawn(move || {
        for msg in rx {
            copyFile.write_all(msg.as_bytes()).unwrap();
        }        
    });

    reader.join().unwrap();
    writer.join().unwrap();
}


fn moveDataWithThread() { 
    let data = vec![1, 2, 3];
    let t = thread::spawn(move || {
        data.iter().for_each(|x| println!("{:?}", x));
    });
    
    t.join().unwrap();
}

fn updateCounter() {
    let counter = Arc::new(Mutex::new(0));
    let (tx, rx) = mpsc::channel();

    let handles: Vec<_> = (0..5)
        .map(|_| {
            let counter = Arc::clone(&counter);
            let msg = tx.clone();
            thread::spawn(move || {
                let mut num = counter.lock().unwrap();
                *num += 1;
                msg.send(format!("Counter: {}", *num)).unwrap();
            })
        })
        .collect();

    drop(tx); // Because original tx still alive so receiving messages will keep waiting since tx still open.
    // In other examples, tx is not cloned and moved into threads so it is dropped once it is moved into the thread, meaning you don't need to drop manually.

    for handle in handles {
        handle.join().unwrap();
    }

    for message in rx {
        println!("{}", message);
    }
    println!(
        "Final counter: {}",
        *counter.lock().unwrap()
    );
}