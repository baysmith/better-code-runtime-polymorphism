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

pub type Document<T> = Vec<T>;

impl<T> Draw for Document<T>
where
    T: Draw,
{
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
