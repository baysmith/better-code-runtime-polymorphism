mod library;
use crate::library::*;

fn main() {
    let stdout = std::io::stdout();
    let mut out = stdout.lock();

    let mut document = Document::new();
    document.reserve(5);

    document.push(Object::new_int(0));
    document.push(Object::new_string("Hello!".to_string()));
    document.push(Object::new_int(2));
    document.push(Object::new_int(3));

    document.reverse();

    document.draw(&mut out, 0).expect("draw document error");
}
