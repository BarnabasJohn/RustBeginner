
/*

fn main() {
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    //the deref pointer->points to the 
    //memory address stored in y to the actual address
    assert_eq!(5, *y);

    //Using y without deref will lead to error since
    //you can't compare int type with reference type
    //assert_eq!(5, y);
}


fn main() {
    let x = 5;
    let y = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
}
*/

use std::ops::Deref;

struct MyBox<T>(T);//custom smart pointer similar to box

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

//the custom pointer cant dereference without Deref lib
//to use an implementation block to dereference 

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}


fn main() {
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *(y.deref()));

    let m = MyBox::new(String::from("Rust"));
    hello(&m);

}

fn hello(name: &str) {
    println!("Hello, {}!", name)
}
