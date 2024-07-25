/*

fn main() {
    let string1 = String::from("abcd");
    let string2 = String::from("xyz");

    let result = longest(string1.as_str(), string2.as_str());
    println!("The longest string is {}", result);
}

 //generic lifetime annotations 'a or 'abcsd--must start with 'a
 //create relationships between references ie the lifetime of the
 //return reference will be the same as the shortest lifetime of
 //argument references

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len(){
        x
    } else {
        y
    }
}

//1. Each parameter that is a reference gets its own lifetime parameter
//2. If there is exactly one input lifetime parameter, that lifetime is
//   assigned to all output lifetime parameters
//3. If there are multiple input lifetime parameters but one of them is
//   &self or &mut self the lifetime of self is assigned to all output 
//   lifetime parameters

//static lifetime is a lifetime annotation that could live as long as the
//whole program


fn main() {
    let s: &'static str = "I have a static lifetime.";
}
*/

use std::fmt::Display;
//Display replresent types that can be printed to the screen
fn longest_with_an_announcement<'a, T> (
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Announcement!! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}


fn main () {

}


