use std::fs;

use libraw::Processor;

#[test]
fn colorchart_5d2_6000k() {
    let buf = fs::read("../samples/images/colorchart-5D2-6000K.dng").expect("read in");

    let processor = Processor::new();
    let decoded = processor.decode(&buf).expect("decoding successful");
    let sizes = decoded.sizes();

    assert_eq!(sizes.width, 5634);
    assert_eq!(sizes.height, 3752);
    assert_eq!(sizes.raw_width, 5792);
    assert_eq!(sizes.raw_height, 3804);
    assert_eq!(
        sizes.raw_width as usize * sizes.raw_height as usize,
        (&decoded).len()
    );
}

#[test]
fn colorchart_eos_7d() {
    let buf = fs::read("../samples/images/colorchart-eos-7d.cr2").expect("read in");

    let processor = Processor::new();
    let decoded = processor.decode(&buf).expect("decoding successful");
    let sizes = decoded.sizes();

    assert_eq!(sizes.width, 5202);
    assert_eq!(sizes.height, 3464);
    assert_eq!(sizes.raw_width, 5360);
    assert_eq!(sizes.raw_height, 3516);
    assert_eq!(
        sizes.raw_width as usize * sizes.raw_height as usize,
        (&decoded).len()
    );
}

#[test]
fn colorchart_iphone7plus_cloudy() {
    let buf = fs::read("../samples/images/colorchart-iphone7plus-cloudy.dng").expect("read in");

    let processor = Processor::new();
    let decoded = processor.decode(&buf).expect("decoding successful");
    let sizes = decoded.sizes();

    // iPhone image has 90-deg clockwise rotation
    assert_eq!(sizes.flip, 6);
    // Width and Height are flipped compared to the process test because
    // when processing, the image is rotated
    assert_eq!(sizes.width, 4032);
    assert_eq!(sizes.height, 3024);
    assert_eq!(sizes.raw_width, 4032);
    assert_eq!(sizes.raw_height, 3024);
    assert_eq!(
        sizes.raw_width as usize * sizes.raw_height as usize,
        (&decoded).len()
    );
}

#[test]
fn komainu() {
    let buf = fs::read("../samples/images/komainu.nef").expect("read in");

    let processor = Processor::new();
    let decoded = processor.decode(&buf).expect("decoding successful");
    let sizes = decoded.sizes();

    assert_eq!(sizes.width, 6034);
    assert_eq!(sizes.height, 4028);
    assert_eq!(sizes.raw_width, 6080);
    assert_eq!(sizes.raw_height, 4028);
    assert_eq!(
        sizes.raw_width as usize * sizes.raw_height as usize,
        (&decoded).len()
    );
}
