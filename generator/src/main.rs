use image::{Rgb, RgbImage};
use imageproc::drawing::{draw_text_mut, text_size};
use rusttype::{Font, Scale};
use std::path::Path;

fn main() {
    let path = Path::new("monitor.bmp");

    let mut image = RgbImage::new(1280, 825);

    let font = Vec::from(include_bytes!("DejaVuSans.ttf") as &[u8]);
    let font = Font::try_from_vec(font).unwrap();

    let height = 80.0;
    let scale = Scale {
        x: height * 2.0,
        y: height,
    };

    let text = "Hello, world!";
    draw_text_mut(&mut image, Rgb([255u8, 255u8, 255u8]), 0, 0, scale, &font, text);
    let (w, h) = text_size(scale, &font, text);
    println!("Text size: {}x{}", w, h);

    let _ = image.save(path).unwrap();
}
