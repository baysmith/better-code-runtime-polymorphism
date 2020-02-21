use std::fmt::Display;
use std::io::Result;
use std::io::Write;

pub trait Concept {
    fn draw(&self, out: &mut dyn Write, position: usize) -> Result<()>;
}

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
}

impl<T: Display> Concept for T {
    default fn draw(&self, out: &mut dyn Write, position: usize) -> Result<()> {
        out.write(" ".repeat(position).as_bytes())?;
        out.write(format!("{}\n", self).as_bytes())?;
        Ok(())
    }
}

pub struct Document {
    contents: Vec<Object>,
}

impl Document {
    pub fn new() -> Self {
        Document {
            contents: Vec::new(),
        }
    }
    pub fn reserve(&mut self, size: usize) {
        self.contents.reserve(size);
    }
    pub fn push(&mut self, item: Object) {
        self.contents.push(item);
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
}
