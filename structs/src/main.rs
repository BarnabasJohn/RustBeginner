
/*
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

//tupple structs
struct Color(i32, i32, i32);

struct Point(i32, i32, i32);


fn main() {
    let mut user1=User{
        email: String::from("barney@mail.com"),
        username: String::from("barney"),
        active: true,
        sign_in_count: 1
    };

    let name = user1.username;

    user1.username = String::from("barney2024");

    let user2 = build_user(
        String::from("Swoop@mail.com"),
        String::from("Indah")
    );

    let user3 = User {
        email: String::from("morio@mail.com"),
        username: String::from("mpweza"),
        ..user2
    };

}

fn build_user(email: String, username: String) -> User {
    User {
        username,
        email,
        sign_in_count: 1,
        active: true,
    }
}
*/

#[derive(Debug)]

struct Rectangle {
    width: u32,
    height: u32
}

//implimentation blocks house methods(functions)
//associated with structs

impl Rectangle {
    fn area(&self) -> u32 {
        self.height * self.width
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

//associated functions of structs

impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size
        }
    }
}

fn main(){
    let rect = Rectangle {
        width: 30,
        height: 50
    };

    let rect1 = Rectangle {
        width: 20,
        height: 40
    };

    let rect2 = Rectangle {
        width: 60,
        height: 90
    };

    let rect3 = Rectangle::square(25);

    println!("rect: {:?}", rect);
    //for custom types like struct, display trait for printing
    //has to be manually set by {:?} or {:#? } 

    println!("rect can hold rect1: {}", rect.can_hold(&rect1));

    println!("rect can hold rect2: {}", rect.can_hold(&rect2));

    println!(
        "The area of the rectangle is {} square pixels",
        rect.area()
        //area(&rect)
    );
}

// fn area(rectangle: &Rectangle) -> u32 {
//     rectangle.width * rectangle.height
// }

