/*

fn main() {
    let a = [1, 2, 3];

    let mut v = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);

    let v2 = vec![1, 2, 3, 4, 5];

    println!("the second element is {}", &v2[1]);

    match v2.get(21) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element.")
    }

    for i in &mut v {
        *i += 50;//dereference operator *, gets the underlying value
    }

    for i in &v {
        println!("{}", i);
    }
}


fn main () {
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String)
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.22)
    ];

    match &row[2] {
        SpreadsheetCell:: Int(i) => println!("{}", i),
        _=> println!("Not an interger")
    }
}



//Strings


fn main() {
    let s1 = String::from("hello, ");
    let s2 = String::from("world!");
    let s3 = format!("{}{}", s1, s2);
    println!("{}", s3)
}


//inorder to access indexes of a string, bytes are used
// since utf8 encoding doesn't suit use of integer as indexes
//as some characters in a string may be 1 or bytes long

use unicode_segmentation::UnicodeSegmentation;


fn main() {
    let s = String::from("lahaula");

    for b in s.bytes() {
        println!("{}", b);
    }

    for c in s.chars() {
        println!("{}", c);
    }

    for g in s.graphemes(true) {
        println!("{}", g);
    }
}





//Hashmaps
use std::collections::HashMap;


fn main() {
    let blue = String::from("Blue");
    let yellow = String::from("Yellow");

    let mut scores = HashMap::new();

    scores.insert(blue, 10);
    scores.insert(yellow, 30);

    for (key, value) in &scores {
        println!("{}: {}", key , value);
    }
}



use std::collections::HashMap;


fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 20);//overwrite

    scores.entry(String::from("Yellow")).or_insert(30);
    scores.entry(String::from("Yellow")).or_insert(30);
    //2nd entry wont overwrite as it checks if key-value pair exists

    for (key, value) in &scores {
        println!("{}: {}", key , value);
    }


}

*/

use std::collections::HashMap;



fn main() {
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}







