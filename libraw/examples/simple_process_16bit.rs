use std::fs::{self, File};
use std::io::Write;

use libraw::Processor;

fn main() {
    let buf = fs::read("raw.RW2").expect("read in");

    let processor = Processor::new();
    let processed = processor
        .process_16bit(&buf)
        .expect("processing successful");

    let mut out = File::create("out_16bit.ppm").expect("create out");
    let header = format!(
        "P6 {} {} {}\n",
        processed.width(),
        processed.height(),
        65535
    );
    out.write_all(header.as_ref()).expect("header");
    // PPM files must be in big endian
    let mut out_vec = Vec::with_capacity(processed.len() * 2);
    for chunk in processed.iter() {
        out_vec.extend_from_slice(&chunk.to_be_bytes());
    }
    out.write_all(&out_vec).expect("pixels");
}
