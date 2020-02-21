use std::io::Result;
use std::io::Write;

type Object = i32;

pub trait Draw {
    fn draw(&self, out: &mut dyn Write, position: usize) -> Result<()>;
}

impl Draw for Object {
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
