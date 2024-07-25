//drop trait is used for customizing what happens when a value is dropped


struct CustomSmartPointer {
    data: String,
}


impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}` !", self.data);
    }
}


/*
fn main() {
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };

    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };

    println!("CustomSmartPointers created.");
}
*/



fn main() {
    let c = CustomSmartPointer {
        data: String::from("some stuff"),
    };

    println!("CustomSmartPointer created.");

    //c.drop(); not allowed
    drop(c); //drop method supplied by rust std lib
    println!("CustomSmartPointer dropped before end of main.");
}