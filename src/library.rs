use std::io::Result;
use std::io::Write;

pub trait Concept {
    fn draw(&self, out: &mut dyn Write, position: usize) -> Result<()>;
}

pub struct Object {
    value: Box<dyn Concept>,
}

impl Object {
    pub fn new_int(x: i32) -> Self {
        Object { value: Box::new(x) }
    }
    pub fn new_string(x: String) -> Self {
        Object { value: Box::new(x) }
    }
}

impl Concept for Object {
    fn draw(&self, out: &mut dyn Write, position: usize) -> Result<()> {
        self.value.draw(out, position)
    }
}

impl Concept for String {
    fn draw(&self, out: &mut dyn Write, position: usize) -> Result<()> {
        out.write(" ".repeat(position).as_bytes())?;
        out.write(format!("{}\n", self).as_bytes())?;
        Ok(())
    }
}

impl Concept for i32 {
    fn draw(&self, out: &mut dyn Write, position: usize) -> Result<()> {
        out.write(" ".repeat(position).as_bytes())?;
        out.write(format!("{}\n", self).as_bytes())?;
        Ok(())
    }
}

pub type Document = Vec<Object>;

impl Concept for Document {
    fn draw(&self, out: &mut dyn Write, position: usize) -> Result<()> {
        out.write(" ".repeat(position).as_bytes())?;
        out.write(b"<document>\n")?;
        for e in self {
            e.draw(out, position + 2)?;
        }
        out.write(" ".repeat(position).as_bytes())?;
        out.write(b"</document>\n")?;
        Ok(())
    }
}
