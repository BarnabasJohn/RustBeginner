use trait_objects::{Draw, Screen, Button };


pub struct SelectBox {
    pub width: u32,
    pub height: u32,
    pub options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        //draw selectbox
    }
}

fn main() {
    let screen = Screen {
        components: vec![
            Box::new(SelectBox{
                width: 100,
                height: 100,
                options: vec![
                    String::from("yes"),
                    String::from("no"),
                    String::from("maybe"),
                ]
            }),
            Box::new(Button {
                width: 100,
                height: 100,
                label: String::from("ok"),
            })
        ]
    };
    screen.run();
}
