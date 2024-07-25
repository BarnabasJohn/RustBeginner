
/*
enum IpAddKind {
    V4(String),
    V6(String)
}

struct IpAdd {
    kind: IpAddKind,
    address: String
}


fn main() {
    let four = IpAddKind::V4;
    let six = IpAddKind::V6;

    let localhost = IpAdd {
        kind: IpAddKind::V4,
        address: String::from("127.0.0.1")
    }; 
}



//Option enum
//Option<i8> refers to null or potentially null variables

enum Option<T> {
    Some(T),
    None,
}

fn main() {
    let some_number = Some(5);
    let some_string = Some("string");

    let x: i8 = 5;
    //let y = Some(5);
    let y = None;

    let sum= x + y.unwrap_or(0);

    // let absent_number: Option<i32> = None;

    println!("{:?}", some_number);

    println!("{}", sum);
}

*/

fn main() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("{:?}", six);
    println!("{:?}", none);

}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
// An laternative to match is 'if let' which matches
// condition and ignores the rest

