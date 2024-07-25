
fn main() {
    /*


    #[derive(Debug)] 
    enum Language {
        English,
        Spanish,
        Russian,
        Japanese,
    }

    //let language = Language::English;

    // match language {
    //     Language::English => println!("Hello, England!"),
    //     Language::Spanish => println!("Hello, Spain!"),
    //     Language::Russian => println!("Hello, Russia!"),
    //     Language::Japanese => println!("Hello, Japan!"),
    // }

    //Matching using catch all statement(_)
    //This matches to anything else not explicitly handled in code

    let language = Language::Japanese;

    match language {
        Language::English => println!("Hello, England!"),
        Language::Spanish => println!("Hello, Spain!"),
        _ => println!("Unsupported language!"),
    }

    //named variable matching

    let language2 = Language::Russian;

    match language2 {
        Language::English => println!("Hello, England!"),
        Language::Spanish => println!("Hello, Spain!"),
        lang => println!("Unsupported language {:?}!", lang),
        //this catch all version puts it in a variable so as to later
        //print the exact pattern that was to be matched
    }


    //Matching using if let expressions
    //=================================
    //if let are used in matching if you specifically care about only one case


    let authorization_status: Option<&str> = None;
    let is_admin = false;
    let group_id: Result<u8, _> = "34".parse();

    if let Some(status) = authorization_status {
        println!("Authorization status {}!", status);
    } else if is_admin {
        println!("Authorization status: admin");
    } else if let Ok(group_id) = group_id {
        if group_id > 30 {
            println!("Authorization status: priviledged");
        } else {
            println!("Authorization status: basic");
        }
    } else {
        println!("Authorization status: guest");
    }
    

    //while let conditional loops
    //===========================

    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{}", top)
    }

    while let None = stack.pop() {
        println!("Lift off!");
        break;
    }
    //stack.pop returns an option type which matches to some and stored
    //as variable top which is printed out. If the stack is empty None
    //is returned from the option and while let will be invalid and exits

    */

    //Refutable & Irrefutable patterns
    //================================

    //refutable patterns are patterns that may or may not match
    //irrefutable are patterns that always match

    //Irrefutable
    let x = 5;

    //refutable
    let x: Option<&str> = None;
    if let Some(x) = x {
        println!("{}", x);
    };

    //function parameters, let statements and for loops can
    //only accept irrefutable patterns

    //while let and if let can accept refutable patterns 
 
    
}
