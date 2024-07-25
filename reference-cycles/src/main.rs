
/*
#[derive(Debug)]


enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}


use std::{cell::RefCell, rc::Rc};

use List::{Cons, Nil};


impl List {
    fn tail(&self)-> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}


fn main() {

    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());

    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b)
    }

    println!("b rc count after changing a= {}", Rc::strong_count(&b));
    println!("a rc count after changing a= {}", Rc::strong_count(&a));

    //Uncomment the next line to see that we have a cycle
    //it will overflow the stack
    //println!("a next item = {:?}", a.tail())

}



//Weak is a version of Rc smart pointer that holds non-owning
//reference to managed location
use std::{cell::RefCell, rc::{Rc, Weak}};


#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

fn main() {

    let leaf = Rc::new(Node{
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    //borrow takes an immutable reference, upgrade converts weak
    //to reference counting pointer
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

    let branch = Rc::new(Node{
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });

    //downgrade converts reference counting pointer to a weak
    //smart pointer since parent expects a weak smart pointer
    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());



    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),//stores no of references with ownership of the value
        Rc::weak_count(&leaf),//stores no of references without ownership of the value
    )
}
*/


use std::{cell::RefCell, rc::{Rc, Weak}};


#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

fn main() {

    let leaf = Rc::new(Node{
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),//stores no of references with ownership of the value
        Rc::weak_count(&leaf),//stores no of references without ownership of the value
    );
    
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

    {
        let branch = Rc::new(Node{
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });

        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        println!(
            "branch strong = {}, weak = {}",
            Rc::strong_count(&branch),//stores no of references with ownership of the value
            Rc::weak_count(&branch),//stores no of references without ownership of the value
        );
        
        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),//stores no of references with ownership of the value
            Rc::weak_count(&leaf),//stores no of references without ownership of the value
        );
        
    }
    
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),//stores no of references with ownership of the value
        Rc::weak_count(&leaf),//stores no of references without ownership of the value
    );
    
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());


}
