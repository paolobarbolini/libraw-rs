use std::fs::{self, File};
use std::io::Write;

use libraw::Processor;

fn main() {
    let buf = fs::read("/home/nikita/Desktop/Rust/libraw-rs/samples/images/colorchart-eos-7d.cr2").expect("read in");

    let mut processor = Processor::new();

    processor.gamma(2.4, 12.92); // before starting processing we can set gamma. See double gamma explaination here https://www.libraw.org/docs/API-datastruct-eng.html
                                                // if gamma values is not set by you processor use defaulf values power 2.222 and slope 4.5 
                                                
    let processed = processor.process_8bit(&buf).expect("processing successful");

    let mut out = File::create("out_8bit.ppm").expect("create out");
    let header = format!("P6 {} {} {}\n", processed.width(), processed.height(), 255);
    out.write_all(header.as_ref()).expect("header");
    out.write_all(&processed).expect("pixels");
}
