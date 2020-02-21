#![feature(specialization)]
mod library;
use crate::library::*;
use std::fmt::{self, Display, Formatter};
use std::io::Write;
use std::thread;
use std::time;

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

    let mut h = History::new();
    h.push(Document::new());

    current(&mut h).push(Object::new(0));
    current(&mut h).push(Object::new("Hello!".to_string()));

    current(&mut h).draw(&mut out, 0).expect("draw error");
    out.write(b"--------------------------\n")
        .expect("write error");

    commit(&mut h);

    current(&mut h)[0] = Object::new(42.5);

    let document = current(&mut h).clone();
    let saving = thread::spawn(move || {
        thread::sleep(time::Duration::from_secs(3));
        let stdout = std::io::stdout();
        let mut out = stdout.lock();
        println!("-------- 'save' --------");
        document.draw(&mut out, 0).expect("draw error");
    });

    current(&mut h)[1] = Object::new("World".to_string());
    // Must assign document copy to a variable to avoid multiple mutable borrow
    // when in expression for push()
    let doc = Object::new(current(&mut h).clone());
    current(&mut h).push(doc);
    current(&mut h).push(Object::new(MyClass {}));

    current(&mut h).draw(&mut out, 0).expect("draw error");
    out.write(b"--------------------------\n")
        .expect("write error");

    undo(&mut h);

    current(&mut h).draw(&mut out, 0).expect("draw error");

    drop(out);
    saving.join().expect("join error");
}
