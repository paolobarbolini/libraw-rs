use std::fs::{self, File};
use std::io::Write;

use libraw::Processor;

fn main() {
    let buf = fs::read("raw.RW2").expect("read in");

    let processor = Processor::new();
    let processed = processor.process(&buf).expect("processing successful");

    let mut out = File::create("out.ppm").expect("create out");
    let header = format!("P6 {} {} {}\n", processed.width(), processed.height(), 255);
    out.write_all(header.as_ref()).expect("header");
    out.write_all(&processed).expect("pixels");
}
