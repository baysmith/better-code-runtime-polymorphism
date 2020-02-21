use std::io::Result;
use std::io::Write;

pub trait Draw {
    fn draw(&self, out: &mut dyn Write, position: usize) -> Result<()>;
}

pub struct Object {
    value: Box<i32>,
}

impl Object {
    pub fn new(x: i32) -> Self {
        Object { value: Box::new(x) }
    }
}

impl Draw for Object {
    fn draw(&self, out: &mut dyn Write, position: usize) -> Result<()> {
        self.value.draw(out, position)
    }
}

impl Draw for i32 {
    fn draw(&self, out: &mut dyn Write, position: usize) -> Result<()> {
        out.write(" ".repeat(position).as_bytes())?;
        out.write(format!("{}\n", self).as_bytes())?;
        Ok(())
    }
}

pub type Document = Vec<Object>;

impl Draw for Document {
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
