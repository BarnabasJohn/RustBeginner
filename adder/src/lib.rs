pub fn add(left: usize, right: usize) -> usize {
    left + right
}



pub fn add_two(a: i32) -> i32 {
    internal_adder(a, 2)
}


fn internal_adder(a:i32, b:i32) -> i32 {
    a + b
}


#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub fn greeting(name: &str) -> String {
    format!("Hello {}!", name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn internal() {
        assert_eq!(4, internal_adder(2, 2));
    }

    //should panic test
    #[test]
    #[should_panic]
    //[should_panic(expected = "name must be carol")]
    fn greeting_name_fail () {
        let result = greeting("carol");
        assert!(
            result.contains("indah"),
            "Greeting did not contain name, value was `{}`", result//custome error message
        )
    }


    #[test]
    fn greeting_name_pass () {
        let result = greeting("carol");
        assert!(
            result.contains("carol"),
            "Greeting did not contain name, value was `{}`", result//custome error message
        )
    }


    #[test]
    #[ignore] //this attribute can be used to skip a test
    fn failing_test() {
        panic!("Make this test fail");
    }



    //assert!() macro evaluates whether an expression
    // evaluate to true

    //assert_eq!() macro evaluates whether an two values
    // are equal its opposite is assert_ne!()

    #[test]
    fn can_hold_test() {
        let larger = Rectangle {
            width: 8,
            height: 7
        };

        let smaller = Rectangle {
            width: 3,
            height: 2
        };

        assert!(larger.can_hold(&smaller))
    }
}
