mod library;
use crate::library::*;
use std::io::Result;
use std::io::Write;

struct MyClass {}

impl Draw for MyClass {
    fn draw(&self, out: &mut dyn Write, position: usize) -> Result<()> {
        out.write(" ".repeat(position).as_bytes())?;
        out.write(b"my_class_t\n")?;
        Ok(())
    }
}

fn main() {
    let stdout = std::io::stdout();
    let mut out = stdout.lock();

    let mut document = Document::new();
    document.reserve(5);

    document.push(Object::new(0));
    document.push(Object::new(1));
    document.push(Object::new(2));
    document.push(Object::new(3));

    document.reverse();

    document.draw(&mut out, 0).expect("draw document error");
}
