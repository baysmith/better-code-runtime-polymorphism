mod library;
use crate::library::*;

fn main() {
    let stdout = std::io::stdout();
    let mut out = stdout.lock();
    let mut document = Document::new();
    document.push(0);
    document.push(1);
    document.push(2);
    document.push(3);
    document.draw(&mut out, 0).expect("draw document error");
}
