/*
fn main() {
    //logic for finding largest numbers in vectors
    let numbers = vec![34, 50, 25, 100, 65];

    let mut largest = numbers[0];

    for number in numbers {
        if number > largest {
            largest = number;
        }
    }

    println!("The largest number is: {}", largest);

    //for the sake of reuseability, use a function for the logic
    // so diffrent vectors can use the same logic to find
    //their largest numbers

    let numbers = vec![102, 500, 250, 300, 650];

    let largest = get_largest_num(numbers);

    println!("The largest number is: {}", largest);

    //if one wants to find the largest character in a vector of
    //characters we'd have to use a totally different function

    let chars = vec!['y', 'm', 'c', 'a'];

    let largest = get_largest_char(chars);

    println!("The largest character is: {}", largest);

    //Generics offer a way to make a function that can take in any
    //argument type and we'd have one function for largest number
    //or character or float etc
    let chars = vec!['y', 'm', 'c', 'a'];

    let numbers = vec![102, 500, 250, 300, 650];

    let largest_char = get_largest(chars);

    let largest_num = get_largest(numbers);

    println!("The largest character is: {}", largest_char);

    println!("The largest number is: {}", largest_num);
}

fn get_largest_num(numbers: Vec<i32>) -> i32 {
    let mut largest = numbers[0];
    for number in numbers {
        if number > largest {
            largest = number;
        }
    }

    largest
}

fn get_largest_char(chars: Vec<char>) -> char {
    let mut largest = chars[0];
    for char in chars {
        if char > largest {
            largest = char;
        }
    }

    largest
}

//T is a generic type, PartialOrd describes a type that can be ordered
fn get_largest<T: PartialOrd + Copy>( argument: Vec<T>) -> T {
    let mut largest = argument[0];
    for element in argument {
        if element > largest {
            largest = element;
        }
    }

    largest
}



struct Point<T> {
    x: T,
    y: T,
}

fn main () {
    let p1 = Point { x:5, y: 10 };
    let p2= Point { x:3.5, y: 1.0 };
    //let p3= Point { x:3.5, y: 10 };
    //p3 won't be a valid point because both x and y
    //should be same type
}

struct Point2<T, U> {
    x: T,
    y: U,
}

fn main () {
    let p1 = Point2 { x:5, y: 10 };
    let p2= Point2 { x:3.5, y: 1.0 };
    let p3= Point2 { x:3.5, y: 10 };
}

enum struct also use generics

enum Option<T>{
    Some(T),
    None
}

enum Result<T, E>{
    Ok(T),
    Err(E)
}



//Generics on structs implementation blocks

struct Point<T> {
    x: T,
    y: T,
}

impl <U> Point<U> {
    fn x(&self) -> &U {
        &self.x
    }
}

fn main () {
    let p1 = Point { x:5, y: 10 };
}

*/




