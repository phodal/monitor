use image::{Rgb};
use imageproc::drawing::{draw_text_mut, text_size};
use rusttype::{Font, Scale};
use std::path::Path;
use image::ImageBuffer;

use serde::{Serialize, Deserialize};
use reqwest::Client;
use std::thread::sleep;
use std::time::Duration;

#[derive(Deserialize, Serialize, Debug)]
struct Quote {
    id: i32,
    quote: String,
    author: String,
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    loop {
        let request_url = format!("https://api.taylor.rest/");
        let client = Client::new();
        let response =
            client.get(&request_url)
                .bearer_auth("")
                .send()
                .await?;

        let quote: Quote = response.json().await?;
        write_text(quote.quote.as_str());

        // for rpi only
        match std::process::Command::new("sudo")
            .arg("epaper")
            .arg("monitor.bmp")
            .status() {
            Ok(_status) => {}
            Err(err) => {
                println!("{:?}", err);
            }
        }

        sleep(Duration::from_secs(60 * 30));
    }

    Ok(())
}

fn write_text(origin: &str) {
    let path = Path::new("monitor.bmp");

    let mut image = ImageBuffer::from_pixel(1280, 825, Rgb([255, 255, 255]));

    let font = Vec::from(include_bytes!("SourceCodePro-Regular.ttf") as &[u8]);
    let font = Font::try_from_vec(font).unwrap();

    let height = 80.0;
    let scale = Scale {
        x: height,
        y: height,
    };

    let sub_len = 32;
    let chars: Vec<char> = origin.chars().collect();
    let subs = &chars.chunks(sub_len)
        .map(|chunk| chunk.iter().collect::<String>())
        .collect::<Vec<_>>();

    let mut index = 0;
    for sub in subs {
        let y = index * 80;
        draw_text_mut(&mut image, Rgb([0u8, 0u8, 0u8]), 0, y, scale, &font, sub.as_str());
        index = index + 1;
    }

    let _ = image.save(path).unwrap();
}
