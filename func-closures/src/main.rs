/*

//Passing functions to functions
fn add_one(x:i32) -> i32 {
    x+1
}

fn do_twice(f: fn(i32)-> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

fn main() {
    let answer = do_twice(add_one, 5);

    println!("The answer is: {}", answer)

}

*/

//Adapting the program so the function can take either
//function or closure
fn add_one(x:i32) -> i32 {
    x+1
}


fn do_twice<T>(f: fn(i32)-> i32, arg: i32) -> i32 
where T: Fn(i32) -> i32 {
    f(arg) + f(arg)
}

fn main() {
    let answer = do_twice(add_one, 5);

    println!("The answer is: {}", answer)

}


