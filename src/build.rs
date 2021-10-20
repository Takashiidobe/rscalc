use man::prelude::*;
use std::fs::File;
use std::io::{Error, Write};

fn main() -> Result<(), Error> {
    let path = "rscalc.1";
    let mut output = File::create(path)?;

    let msg = Manual::new("rscalc")
        .about("A cli based calculator in rust.")
        .arg(Arg::new("path"))
        .example(Example::new().text("Running the program").command("rscalc"))
        .custom(Section::new("Features").paragraph(
            r#"Operators
    - `+` for Addition
    - `-` for Subtraction
    - `*` for Multiplication
    - `/` for Division
    - `^` for Power"#,
        ))
        .author(Author::new("Takashi I").email("mail@takashiidobe.com"))
        .render();

    write!(output, "{}", msg)
}
