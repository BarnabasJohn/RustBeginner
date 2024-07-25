use std::{thread, time::Duration};


/*
fn main() {
    thread::spawn(|| {
        for i in 1..10{
            println!("Hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(2));
        }
    });

    for i in 1..5{
        println!("Hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(2));
    }

    //in this program, once the main thread is done, the program
    //exits whether spawn thread is completed or not
}


fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10{
            println!("Hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(2));
        }
    });

    //handle.join().unwrap();
    //this will run spawned thread to completion then start main thread

    for i in 1..5{
        println!("Hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(2));
    }

    handle.join().unwrap();
    //putting the result of spawned thread in a variable and
    //using join() to join it to the main thread, prevents the main
    //thread from exiting until handle completes
    // unwrap is used because join returns a result type

}
*/


fn main() {
    let v = vec![1, 2, 3, 4];

    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();
}

