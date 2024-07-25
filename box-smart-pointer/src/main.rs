/*

//This program won't run as the list is a recurvise type having
//infinite size

//Hence Box smart pointers aid in that they point to the value
//in the heap instead holding the actual value

enum List {
    Cons(i32, List),
    Nil,
}


use List::{Cons, Nil};

fn main() {
    let list = Cons(1, Cons(2, Cons(3, Nil)));
}
*/

enum List {
    Cons(i32, Box<List>),
    Nil,
}

use List::{Cons, Nil};

fn main() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}

