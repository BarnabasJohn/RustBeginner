//design pattern of rust that allows one to mutate data
//even when there are immutable references to that data
#[derive(Debug)]


enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}


use std::{cell::RefCell, rc::Rc};

use List::{Cons, Nil};

fn main() {

    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));

    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    //  * derefence is called so as to mutate the underlying value
    // borrow_mut mthd of RefCell is for mutatating the value
    *value.borrow_mut() += 10;

    println!("a after = {:?}", a);

    println!("b after = {:?}", b);

    println!("c after = {:?}", c);

}
