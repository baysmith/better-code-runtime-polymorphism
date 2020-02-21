mod library;
use crate::library::*;

#[allow(dead_code)]
struct Some {
    member: Object,
}

fn func() -> Some {
    Some {
        member: Object::new(5),
    }
}

#[allow(unused_assignments, unused_variables)]
fn main() {
    /*
        Quiz: what will this print?
    */
    let mut x = Some {
        member: Object::new(0),
    };

    x = func();
}
