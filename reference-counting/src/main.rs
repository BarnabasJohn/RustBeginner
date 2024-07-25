//reference counting smart pointers are pointers that allow
//values to have multiple owners while keeping track of the owners
//and when the value has no owners, the value gets cleaned

/*
enum List {
    Cons(i32, Box<List>),
    Nil,
}


use List::{Cons, Nil};
fn main() {
    let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));
    let b = Cons(3, Box::new(a));
    let c = Cons(4, Box::new(a));//program won't run
    //because Con takes ownership of a, so b owns a, c cant use it
}

//the above issue can be solved using reference counting pointers



enum List {
    Cons(i32, Rc<List>),
    Nil,
}


use std::rc::Rc;

use List::{Cons, Nil};
fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    let b = Cons(3, Rc::clone(&a));
    let c = Cons(4, Rc::clone(&a));
}
*/  



enum List {
    Cons(i32, Rc<List>),
    Nil,
}


use std::rc::Rc;

use List::{Cons, Nil};
fn main() {
    //Rc::strong_count gives the actual count of value ownerships
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));

}

