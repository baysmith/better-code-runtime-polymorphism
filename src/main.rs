#![feature(specialization)]
mod library;
use crate::library::*;
use std::fmt::{self, Display, Formatter};

#[derive(Clone)]
struct MyClass {}

impl Display for MyClass {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "my_class_t")
    }
}

fn main() {
    let stdout = std::io::stdout();
    let mut out = stdout.lock();

    let mut document = Document::new();
    document.reserve(5);

    document.push(Object::new(0));
    document.push(Object::new("Hello!".to_string()));
    document.push(Object::new(document.clone()));
    document.push(Object::new(MyClass {}));

    document.draw(&mut out, 0).expect("draw document error");
}
