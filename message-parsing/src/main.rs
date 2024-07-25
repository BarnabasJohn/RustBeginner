use std::sync::mpsc;
use std::thread;
use std::time::Duration;

/*
fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let msg = String::from("hi");
        tx.send(msg).unwrap();
    });

    let received = rx.recv().unwrap();
    //recv() will block main thread execution while it awaits a message
    //to be sent down the channel
    //try_recv will not block main thread execution and will return a result
    //type immediately or error if no value present
    println!("Got: {}", received);
}


//sending Multiple messages from thread(producer)

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("Hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
           tx.send(val).unwrap(); 
           thread::sleep(Duration::from_secs(1))
        }
        
    });

    for received in rx {
       println!("Got: {}", received); 
    }
    
}
*/

//Sending messages from multiple threads(producers)

fn main() {
    let (tx, rx) = mpsc::channel();

    let tx2 = tx.clone();

    thread::spawn(move || {
        let vals = vec![
            String::from("Hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
           tx.send(val).unwrap(); 
           thread::sleep(Duration::from_secs(1))
        }
        
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
           tx2.send(val).unwrap(); 
           thread::sleep(Duration::from_secs(1))
        }
        
    });

    for received in rx {
       println!("Got: {}", received); 
    }
    
}

