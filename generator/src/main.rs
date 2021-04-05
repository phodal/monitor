use image::{Rgb};
use imageproc::drawing::{draw_text_mut, text_size};
use rusttype::{Font, Scale};
use std::path::Path;
use image::ImageBuffer;

fn main() {
    let path = Path::new("monitor.bmp");

    let mut image = ImageBuffer::from_pixel(1280, 825, Rgb([255, 255, 255]));

    let font = Vec::from(include_bytes!("wqy-microhei.ttc") as &[u8]);
    let font = Font::try_from_vec(font).unwrap();

    let height = 80.0;
    let scale = Scale {
        x: height,
        y: height,
    };

    let text = "你好，世界。";
    draw_text_mut(&mut image, Rgb([0u8, 0u8, 0u8]), 0, 0, scale, &font, text);
    let (w, h) = text_size(scale, &font, text);
    println!("Text size: {}x{}", w, h);

    let _ = image.save(path).unwrap();
}
