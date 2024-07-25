/*
fn main() {
    
    let x = Some(5);

    let y = 10;

    match x {
        Some(50) => println!("got 50"),
        Some(y) => println!("Matched, y = {:?}", y),//named variable matching, the y is not
        //related to variable y above
        _ => println!("Default case, x= {:?}", x),//will match None
    }
    

    //Matching ranges
     
    let x = 5;

    match x {
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }

    let x = 'c';

    match x {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }
    

    //Matching structs

    struct Point {
        x: i32,
        y: i32,
    }

    fn main () {
        let p = Point{ x:0, y:7 };

        let Point{ x:a, y:b } = p;
        assert_eq!(0, a);
        assert_eq!(7, b);
    }

    


    //Matching enums

    enum Message {
        Quit,
        Move {x: i32, y: i32 },
        Write(String),
        ChangeColor(Color),
    }

    fn main () {
        let msg = Message::ChangeColor(0, 160, 255)

        match msg {

            Message::Quit => {
                println!("Quit");
            }

            Message::Move { x, y } => {
                println!(
                    "Move to x: {} y: {}",
                    x, y
                );
            }

            Message::Write(text) => {
                println!("Text message: {}", text)
            }

            Message::ChangeColor(r, g, b) => {
                println!(
                    "Change color: red {}, green {}, and blue {}",
                    r, g, b
                );
            }
        }
    }
}
    


enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

enum Message {
    Quit,
    Move {x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
}

fn main () {
    let msg = Message::ChangeColor(
        Color::Hsv(0, 160, 255)
    );

    match msg {

        Message::ChangeColor(Color::Rgb(r, g, b)) => {
            println!(
                "Change color: red {}, green {}, and blue {}",
                r, g, b
            );
        }

        Message::ChangeColor(Color::Hsv(h, s, v)) => {
            println!(
                "Change color: hue {}, saturation {}, and value  {}",
                h, s, v
            );
        }

        _=> (),
    }
}



fn main() {
    foo(3, 4);
}

fn foo(_: i32, y: i32) {
    println!("This code only uses the y parameter: {}", y);
}
//the (_) can be used as a function parameter to ignore the parameter
//and use the remainder parameter(s)



//Match guard
//Are if statements used to add conditions in matches

fn main () {
    let num = Some(4);

    match num {
        Some(x) if x < 5 => println!("less than five: {}", x)
        Some(x) => println!("{}", x),
        None => (),
    }
}
*/

// the @ variant
//used when creating variable during matching

fn main () {
    enum Message {
        Hello { id: i32 },
    }

    let msg = Message::Hello { id: 5 };

    match msg {
        Message::Hello {
            id: id_variable @ 3..=7,
        } => println!("Found an id in range: {}", id_variable),

        Message::Hello {
            id: 10..=13,
        } => println!("Found an id in another range"),
        
        Message::Hello {id} =>
        println!("Found some other id: {}", id),
        
    }
}

 
