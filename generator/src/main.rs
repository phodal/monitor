use image::{Rgb};
use imageproc::drawing::{draw_text_mut, text_size};
use rusttype::{Font, Scale};
use std::path::Path;
use image::ImageBuffer;

use serde::{Serialize, Deserialize};
use reqwest::Client;

#[derive(Deserialize, Serialize, Debug)]
struct Quote {
    id: i32,
    quote: String,
    author: String,
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let request_url = format!("https://api.taylor.rest/");
    let client = Client::new();
    let response =
        client.get(&request_url)
            .bearer_auth("")
            .send()
            .await?;

    let quote: Quote = response.json().await?;
    write_text(quote.quote.as_str());

    Ok(())
}

fn write_text(text: &str) {
    let path = Path::new("monitor.bmp");

    let mut image = ImageBuffer::from_pixel(1280, 825, Rgb([255, 255, 255]));

    let font = Vec::from(include_bytes!("wqy-microhei.ttc") as &[u8]);
    let font = Font::try_from_vec(font).unwrap();

    let height = 40.0;
    let scale = Scale {
        x: height,
        y: height,
    };

    draw_text_mut(&mut image, Rgb([0u8, 0u8, 0u8]), 0, 0, scale, &font, text);
    let (w, h) = text_size(scale, &font, text);
    println!("Text size: {}x{}", w, h);

    let _ = image.save(path).unwrap();
}
