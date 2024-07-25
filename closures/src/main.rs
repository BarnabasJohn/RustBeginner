//closures are unnamed functions ie anonymous functions
//closure parameters are in pipes instead of parenthesis

/*

fn main() {

    //let clzr = || { x + 1 };

    //let clzr = |x| { x + 1 };

    //let clzr = |x| x + 1 ;

    let clzr = |num| num + 1;
    println!("number is {}", clzr(1));

    let myvalue = 5;
    let clzr2 = |num| num + myvalue;
    println!("number is {}", clzr2(10));

}

*/

/*
use std::thread;
use std::time::Duration;


fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity

}


fn main() {
    let simulated_intensity = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_intensity, simulated_random_number);



}


//structs that use closures need to have generics and trait bounds
struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculated: T,
    value: Option<u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculated: calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculated)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

fn generate_workout(intensity: u32, random_number: u32) {
    
    let mut cached_results = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });
    
    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            cached_results.value(intensity)
        );
        println!(
            "Next, do {} situps!",
            cached_results.value(intensity)
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                cached_results.value(intensity)
            );
        }
    }
}



fn main() {
    let x = 4;

    //closure has access to environment variable
    //i.e closure has access to x
    let equal_to_x = |z| z==x;


    //functions doesn't have access to environment variable
    //i.e function can't access x
    
    // fn equal_to_x2(z: i32)-> bool {
    //     z==x
    // }

    let y = 4;

    assert!(equal_to_x(y)); 
}
*/





//FnOnce, FnMut, Fn
//FnOnce takes ownership of variable inside closures, hence
//these closures are only called once

//FnMut mutably borrows values of variable used in closures

//Fn immutably borrows values of variable used in closures

fn main() {

    //Fn Trait
    let str1 = "the quick brown fox";
    let clzr1 = |x| println!("{} {}", str1, x);
    clzr1("jumps over the lazy dog");

    //FnMut Trait
    let mut str2 = "the quick brown fox".to_string();
    let str3 = String::from("jumps over the lazy dog2");
    let mut clzr2 = |y| str2.push_str(y);
    clzr2(&str3);
    println!("{}", str2);

    //FnOnce
    let str4 = "the quick brown fox";
    let clzr3 = || drop(str4);
    clzr3();



    // let x = vec![1, 2, 3];

    // //let equal_to_x = |z| z==x;

    // let equal_to_x = move |z| z==x;

    // println!("can't use x here: {:?}", x);

    // let y = vec![1, 2, 3];

    // assert!(equal_to_x(y));
}