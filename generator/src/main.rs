use image::{Rgb, ImageBuffer};
use imageproc::drawing::{draw_text_mut};
use rusttype::{Font, Scale};
use std::path::Path;

use serde::{Serialize, Deserialize};
use reqwest::Client;
use std::thread::sleep;
use std::time::Duration;
use chrono::prelude::*;
use std::alloc::Global;

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

        let mut image = ImageBuffer::from_pixel(1280, 825, Rgb([255, 255, 255]));
        draw_image(quote.quote.as_str(), &mut image);
        let _ = image.save(Path::new("monitor.bmp")).unwrap();

        execute_command();
    }
}

#[cfg(target_os = "macos")]
fn execute_command() {
    sleep(Duration::from_secs(10));
}

#[cfg(target_os = "linux")]
fn execute_command() {
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

fn draw_image(origin: &str, image: &mut ImageBuffer<Rgb<u8>, Vec<u8>>) {
    let font = read_font();


    let time_height = draw_time(image, &font);

    draw_sentence(origin, image, &font, time_height)
}

fn draw_sentence(text: &str, image: &mut ImageBuffer<Rgb<u8>, Vec<u8>>, font: &Font, offset: i32) {
    let sub_len = 31;
    let subs = text_to_vec(text, sub_len);
    let mut index = 0;
    let main_scale = Scale { x: 80.0, y: 80.0 };
    for sub in subs {
        let y = index * 80 + offset;
        draw_text_mut(image, Rgb([0u8, 0u8, 0u8]), 0, y, main_scale, &font, sub.as_str());
        index = index + 1;
    }
}

fn draw_time(image: &mut ImageBuffer<Rgb<u8>, Vec<u8>>, font: &Font) -> i32 {
    let small_scale = Scale { x: 40.0, y: 40.0 };

    let time = time_now();
    draw_text_mut(image, Rgb([0u8, 0u8, 0u8]), 0, 0, small_scale, &font, time.as_str());

    let offset = 40;
    offset
}

fn read_font() -> Font<'static> {
    let str = include_bytes!("SourceCodePro-Regular.ttf") as &[u8];
    let font = Vec::from(str);
    let font = Font::try_from_vec(font).unwrap();
    font
}

fn text_to_vec(origin: &str, sub_len: usize) -> Vec<String> {
    let chars: Vec<char> = origin.chars().collect();
    let subs = &chars.chunks(sub_len)
        .map(|chunk| chunk.iter().collect::<String>())
        .collect::<Vec<_>>();

    subs.to_vec()
}

fn time_now() -> String {
    let utc: DateTime<Local> = Local::now();
    let delayed_format = utc.format("%Y-%m-%d %H:%M:%S");

    format!("updated time: {}", delayed_format.to_string())
}
