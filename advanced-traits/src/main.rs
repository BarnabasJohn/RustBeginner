//Associated types allow us to specify a type without
//a concrete value like generics, except associated type
//can only have one concrete value type at a time unlike generics

//type Item;
/* 
pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}

struct Counter {}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        Some(0)
    }
}

//The implementation won't be valid as the associated type
//is different from the initial value

// impl Iterator for Counter {
//     type Item = u16;

//     fn next(&mut self) -> Option<Self::Item> {
//         Some(0)
//     }

*/



fn main() {
    
    println!("Hello, world!");
}