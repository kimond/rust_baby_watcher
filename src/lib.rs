extern crate x264;
extern crate rscam;
extern crate gstreamer;
extern crate glib;

use rscam::{Camera, Config};
use std::fs::File;
use std::io::prelude::*;
use x264::{Encoding, Image, Setup, Preset, Tune};

pub fn start_stream() {
    let mut camera = Camera::new("/dev/video0").unwrap();

    camera.start(&Config {
        interval: (1, 30),
        resolution: (1280, 720),
        format: b"RGB3",
        ..Default::default()
    }).unwrap();


    let mut encoder =
        Setup::preset(Preset::Ultrafast, Tune::None, false, false)
            .fps(30,1)
            .build(Encoding::RGB, 1280, 720)
            .unwrap();
    let mut file = File::create("fade.h264").unwrap();

    println!("Initialized!");

    {
        let headers = encoder.headers().unwrap();
        file.write_all(headers.entirety()).unwrap();
    }

    for i in 0..300 {
        let frame = camera.capture().unwrap();
        let image =  Image::rgb(1280 as i32, 720 as i32, &frame);
        let (data, _) = encoder.encode(i as i64, image).unwrap();
        file.write_all(data.entirety()).unwrap();
    }

    {
        let mut flush = encoder.flush();
        while let Some(result) = flush.next() {
            let (data, _) = result.unwrap();
            file.write_all(data.entirety()).unwrap();
        }
    }
}
