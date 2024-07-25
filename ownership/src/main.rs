
    // ------Ownership Rules --------
    //1. Each value in Rust has a variable that's called it's owner.
    //2. There can only be one owner at a time.
    //3. When the owner goes out of scope, the value will be dropped.
    

    /*{
        // s is not valid here, it's not yet declared
            let s = String::from("hello");//s is valid from this point
        //do stuff with s
        //this scope is now over, and s is no longer valid
    }*/



/*
fn main (){
    let x = 5;
    let y = x; //copy

    let s1 = String::from("hellos");
    //let s2 = s1 //move not copy
    let s2 = s1.clone();//copy

    println!("{}, world", s1);

}
*/



/*
fn main(){
    let s = String::from("hello");
    takes_ownership(s);
    println!("{}", s);//s is already moved to some string

    let x = 5;
    makes_copy(x);//x is copied
    println!("{}", x)//x is available
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}
*/



/*
fn main () {
    let s1 = gives_ownership();
    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2);
    println!("s1 = {}, s3 = {}", s1, s3);
}


fn gives_ownership() -> String {
    let some_string = String:: from("well");

    some_string
}

fn takes_and_gives_back(a_string:String) -> String {
    a_string
}
*/




/*
//References

fn main () {
    let s1 = String::from("hello");
    let len: usize = calculate_length(&s1); //by using & we reference the variable hence no moving
    println!("The length of '{}' is {}", s1, len);
}
//references dont take ownership and are immutable
fn calculate_length(s: &String) -> usize {
    //s.push_str("oops");
    // s cant be changed, references are immutable
    let length = s.len(); //.len() returns length of a string
    length
}
*/


/*
//Mutable references
fn main() {
    let mut s1 = String::from("hello");
    let some = change( &mut s1);
    println!("{}", some)
}

fn change( some_string: &mut String) -> &mut String {
    some_string.push_str(" chapati");
    some_string
}

//two instances of immutable references can occur at some point
// but not mutable references because the data might change leading to corrupt data
*/




/*
//Dangling references->references that point to nothing
fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> &String {
    let s = String::from("hello");

    &s
}
// once dangle has executed the value of s will be dropped
// the reference will point to nothing

----------The Rules Of References-------------
1. At any given time, you can have either one mutable reference
or any number of immutable references
2. References must always be valid

*/




/*
//Slices
fn main() {
    let mut s = String::from("hello world");
    first_word = &s[..5];
    let word = first_word(&s);
    s.clear();
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for(i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}
//enumerate() - allows one to access values of arrays
// as_bytes - Returns a byte slice of this String 's contents.
// b indicates the string is of byte type not unicode

fn main(){
    let a = [1, 2, 3, 4, 5];
    let sliced = &a[..2]
}
*/
