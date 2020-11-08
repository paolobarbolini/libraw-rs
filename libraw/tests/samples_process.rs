use std::fs;

use libraw::Processor;

macro_rules! process {
    ($method: ident) => {
        mod $method {
            use super::*;

            #[test]
            fn colorchart_5d2_6000k() {
                let buf = fs::read("../samples/images/colorchart-5D2-6000K.dng").expect("read in");

                let processor = Processor::new();
                let processed = processor.$method(&buf).expect("processing successful");
                assert_eq!(processed.width(), 5634);
                assert_eq!(processed.height(), 3752);
                assert_eq!(
                    processed.len(),
                    (processed.width() * processed.height() * 3) as usize
                );
            }

            #[test]
            fn colorchart_eos_7d() {
                let buf = fs::read("../samples/images/colorchart-eos-7d.cr2").expect("read in");

                let processor = Processor::new();
                let processed = processor.$method(&buf).expect("processing successful");
                assert_eq!(processed.width(), 5202);
                assert_eq!(processed.height(), 3464);
                assert_eq!(
                    processed.len(),
                    (processed.width() * processed.height() * 3) as usize
                );
            }

            #[test]
            fn colorchart_iphone7plus_cloudy() {
                let buf = fs::read("../samples/images/colorchart-iphone7plus-cloudy.dng")
                    .expect("read in");

                let processor = Processor::new();
                let processed = processor.$method(&buf).expect("processing successful");
                assert_eq!(processed.width(), 3024);
                assert_eq!(processed.height(), 4032);
                assert_eq!(
                    processed.len(),
                    (processed.width() * processed.height() * 3) as usize
                );
            }

            #[test]
            fn komainu() {
                let buf = fs::read("../samples/images/komainu.nef").expect("read in");

                let processor = Processor::new();
                let processed = processor.$method(&buf).expect("processing successful");
                assert_eq!(processed.width(), 6034);
                assert_eq!(processed.height(), 4028);
                assert_eq!(
                    processed.len(),
                    (processed.width() * processed.height() * 3) as usize
                );
            }
        }
    };
}

process!(process_8bit);
process!(process_16bit);
