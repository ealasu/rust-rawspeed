#[macro_use]
extern crate bencher;
extern crate rawspeed;
extern crate image;

use std::fs::File;
use std::io::prelude::*;
use bencher::Bencher;
use rawspeed::{CameraMetadata, RawImage};
use image::Image;

fn decode_raw_image(bench: &mut Bencher) {
    let meta = CameraMetadata::default().unwrap();
    bench.iter(|| {
        let mut file = File::open("test_data/test.cr2").unwrap();
        let mut data = Vec::new();
        file.read_to_end(&mut data).unwrap();
        RawImage::decode(&data, &meta).unwrap()
    })
}

//fn dcraw(bench: &mut Bencher) {
    //bench.iter(|| {
        //Image::<u16>::open_raw("test_data/test.cr2")
    //})
//}

benchmark_group!(benches
                 ,decode_raw_image
                 //,dcraw
                 );
benchmark_main!(benches);
