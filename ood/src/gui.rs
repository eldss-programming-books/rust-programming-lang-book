pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    // The vector of Box<dyn Draw>> types acts as an array containing
    // interface types. The program only needs to know that each type
    // can use the function draw().
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!("drawing button");
    }
}

pub struct SelectBox {
    pub width: u32,
    pub height: u32,
    pub options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!("drawing select-box");
    }
}
