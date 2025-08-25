/// This is simple converion without metadata
use std::fs::{self, File};
use std::io::Write;

use libraw::Processor;

fn main() {
    let buf = fs::read("/home/nikita/Desktop/Rust/libraw-rs/samples/images/colorchart-eos-7d.cr2")
        .expect("read in");

    let mut processor = Processor::new();

    processor.gamma(2.4, 12.92); // before starting processing we can set gamma. See double gamma explaination here https://www.libraw.org/docs/API-datastruct-eng.html
                                 // if gamma values is not set by you processor use defaulf values power 2.222 and slope 4.5

    let processed = processor.process_8bit(&buf).expect("processing successful");

    let to_compress_data: Vec<u8> = Vec::new(); // new variable for compression

    let mut encode = mozjpeg::Compress::new(mozjpeg::ColorSpace::JCS_EXT_RGB);
    encode.set_size(processed.width() as usize, processed.height() as usize);
    encode.set_quality(100.0);

    let mut comp = encode
        .start_compress(to_compress_data)
        .expect("Starting compression error!");
    let pixels = processed.pixels().unwrap();
    //let pixels = &*processed;  // another way to make link on data
    comp.write_scanlines(&pixels[..]).unwrap();

    let compressed = comp.finish().unwrap();

    let mut out = File::create("no_metadata_image.jpg").expect("create out");
    out.write_all(&compressed).expect("pixels");
}
