use std::fs;

use libraw::Processor;

#[test]
fn colorchart_5d2_6000k() {
    let buf = fs::read("../samples/images/colorchart-5D2-6000K.dng").expect("read in");

    let processor = Processor::new();
    let decoded = processor.process(&buf).expect("decoding successful");
    assert_eq!(decoded.width(), 5634);
    assert_eq!(decoded.height(), 3752);
}

#[test]
fn colorchart_eos_7d() {
    let buf = fs::read("../samples/images/colorchart-eos-7d.cr2").expect("read in");

    let processor = Processor::new();
    let decoded = processor.process(&buf).expect("decoding successful");
    assert_eq!(decoded.width(), 5202);
    assert_eq!(decoded.height(), 3464);
}

#[test]
fn colorchart_iphone7plus_cloudy() {
    let buf = fs::read("../samples/images/colorchart-iphone7plus-cloudy.dng").expect("read in");

    let processor = Processor::new();
    let decoded = processor.process(&buf).expect("decoding successful");
    assert_eq!(decoded.width(), 3024);
    assert_eq!(decoded.height(), 4032);
}

#[test]
fn komainu() {
    let buf = fs::read("../samples/images/komainu.nef").expect("read in");

    let processor = Processor::new();
    let decoded = processor.process(&buf).expect("decoding successful");
    assert_eq!(decoded.width(), 6034);
    assert_eq!(decoded.height(), 4028);
}
