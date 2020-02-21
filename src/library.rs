use std::io::Result;
use std::io::Write;

type Object = i32;

fn draw_object(x: &Object, out: &mut dyn Write, position: usize) -> Result<()> {
    out.write(" ".repeat(position).as_bytes())?;
    out.write(format!("{}\n", x).as_bytes())?;
    Ok(())
}

pub type Document = Vec<Object>;

pub fn draw(x: &Document, out: &mut dyn Write, position: usize) -> Result<()> {
    out.write(" ".repeat(position).as_bytes())?;
    out.write(b"<document>\n")?;
    for e in x {
        draw_object(e, out, position + 2)?;
    }
    out.write(" ".repeat(position).as_bytes())?;
    out.write(b"</document>\n")?;
    Ok(())
}
