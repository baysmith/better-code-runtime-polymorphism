use std::fmt::Display;
use std::io::Result;
use std::io::Write;
use std::ops::{Index, IndexMut};

pub trait Concept {
    fn draw(&self, out: &mut dyn Write, position: usize) -> Result<()>;
    fn box_clone(&self) -> Box<dyn Concept>;
}

impl Clone for Box<dyn Concept> {
    fn clone(&self) -> Box<dyn Concept> {
        println!("copy");
        self.box_clone()
    }
}

#[derive(Clone)]
pub struct Object {
    value: Box<dyn Concept>,
}

impl Object {
    pub fn new<T: Concept + 'static>(x: T) -> Self {
        Object { value: Box::new(x) }
    }
}

impl Concept for Object {
    fn draw(&self, out: &mut dyn Write, position: usize) -> Result<()> {
        self.value.draw(out, position)
    }
    fn box_clone(&self) -> Box<dyn Concept> {
        Box::new(self.clone())
    }
}

impl<T: Display + Clone + 'static> Concept for T {
    default fn draw(&self, out: &mut dyn Write, position: usize) -> Result<()> {
        out.write(" ".repeat(position).as_bytes())?;
        out.write(format!("{}\n", self).as_bytes())?;
        Ok(())
    }
    fn box_clone(&self) -> Box<dyn Concept> {
        Box::new(self.clone())
    }
}

#[derive(Clone)]
pub struct Document {
    contents: Vec<Object>,
}

impl Document {
    pub fn new() -> Self {
        Document {
            contents: Vec::new(),
        }
    }
    #[allow(dead_code)]
    pub fn reserve(&mut self, size: usize) {
        self.contents.reserve(size);
    }
    pub fn push(&mut self, item: Object) {
        self.contents.push(item);
    }
}

impl Index<usize> for Document {
    type Output = Object;

    fn index(&self, index: usize) -> &Self::Output {
        &self.contents[index]
    }
}

impl IndexMut<usize> for Document {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.contents[index]
    }
}

impl Concept for Document {
    fn draw(&self, out: &mut dyn Write, position: usize) -> Result<()> {
        out.write(" ".repeat(position).as_bytes())?;
        out.write(b"<document>\n")?;
        for e in &self.contents {
            e.draw(out, position + 2)?;
        }
        out.write(" ".repeat(position).as_bytes())?;
        out.write(b"</document>\n")?;
        Ok(())
    }
    fn box_clone(&self) -> Box<dyn Concept> {
        Box::new(self.clone())
    }
}

pub type History = Vec<Document>;

pub fn commit(x: &mut History) {
    assert!(x.len() > 0);
    let last = x.last_mut().unwrap().clone();
    x.push(last);
}

pub fn undo(x: &mut History) {
    assert!(x.len() > 0);
    x.pop();
}

pub fn current(x: &mut History) -> &mut Document {
    assert!(x.len() > 0);
    x.last_mut().unwrap()
}
