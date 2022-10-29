use std::io::{BufWriter, Write};


fn main() {
    let mut ist: Vec<i16> = Vec::new();
    let mut out = std::fs::File::create("test.txt")?;
    let mut buf = BufWriter::new(out);
    for n in 1..100000000 {
        // println!("{}", n);
        // writeln!(out, "{}", n);
        writeln!(buf, "{}", line)?;
        ist.push(9999);
    }
    buf.flush()?;
}
#![allow(unused)]
fn main() {
fn blah() -> Result<(), std::io::Error> {
let lines = vec!["one", "two", "three"];
use std::io::{BufWriter, Write};
let mut out = std::fs::File::create("test.txt")?;
let mut buf = BufWriter::new(out);
for line in lines {
    writeln!(buf, "{}", line)?;
}
buf.flush()?;
Ok(())
}
}

