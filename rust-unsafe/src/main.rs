/*
fn main() {

    //immutable raw pointers *const <type>
    //mutable raw pointer *mut <type>
    //raw pointers ignore borrowing rules, can be null and
    //aren't automatically cleaned

    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    unsafe fn dangerous() {};

    //inorder to reference raw pointers, we have to use unsafe
    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);

        dangerous();//unsafe functions can only be called in 
        //unsafe blocks
    }
}
*/

//extern is away for code written in for languages to be executed
//when the block is passed to that language

extern "C" {
    fn abs (input: i32) -> i32;
}

fn main () {
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3))
    }
}
