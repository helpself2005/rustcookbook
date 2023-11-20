use std::sync::mpsc;
use std::thread;
use std::time::Duration;


fn spawn_function() {
    for i in 0..5 {
        println!("spawned thread print {}", i);
        thread::sleep(Duration::from_millis(1));
    }
}

fn main() {
    //thread::spawn(spawn_function);
    /*
    thread::spawn(|| {
        for i in 0..5 {
            println!("spawned thread print {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    */

    /*
    let inc = |num: i32| -> i32 {
        num + 1
    };
    println!("inc(5) = {:?}", inc(5));
    */

    /*
    let inc = |num| {
        num + 1
    };

    println!("inc(5) = {:?}", inc(5));
    */

    /*
    let handle = thread::spawn(|| {
        for i in 0..5 {
            println!("spawned thread print {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 0..3 {
        println!("main thread print {}", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();

     */
    /*
    let s = "shipl";

    let handle = thread::spawn(move || {
        println!("hello {}", s);
    });

    handle.join().unwrap();

     */

    let (tx, rx) = mpsc::channel();

    let handle = thread::spawn(move || {
        for i in 0..3 {
            println!("main thread print {}", i);
            let val = i.to_string();
            tx.send(val).unwrap();
        }
    });

    let received = rx.recv().unwrap();
    println!("Got hello : {}", received);
    handle.join().unwrap();
}
