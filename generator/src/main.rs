use std::path::Path;
use std::thread::sleep;
use std::time::Duration;

use chrono::prelude::*;
use image::{ImageBuffer, Rgb};
use imageproc::drawing::{draw_text_mut, text_size};
use reqwest::Client;
use rusttype::{Font, Scale};
use serde::{Deserialize, Serialize};

pub mod monitor_canvas;

#[derive(Deserialize, Serialize, Debug)]
struct Quote {
    id: i32,
    quote: String,
    solution: String,
    author: String,
}

#[derive(Deserialize, Serialize, Debug)]
struct Position {
    x: u32,
    y: u32,
}

const FONT_BYTES: &'static [u8] = include_bytes!("wqy-microhei.ttc");
const WIDTH: u32 = 1280;
const HEIGHT: u32 = 825;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    loop {
        let request_url = format!("https://phodal.github.io/monitor-api/api.json");
        let client = Client::new();
        let response =
            client.get(&request_url)
                .bearer_auth("")
                .send()
                .await?;
        let quote: Quote = response.json().await?;

        let mut image = ImageBuffer::from_pixel(WIDTH, HEIGHT, Rgb([255, 255, 255]));
        draw_image(quote, &mut image);
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

fn draw_image(quote: Quote, image: &mut ImageBuffer<Rgb<u8>, Vec<u8>>) {
    let font = read_font();

    let time = time_now();
    let time_size = 40;
    draw_time(time.as_str(), time_size, image, &font, 0);

    let text_size = 80;
    draw_sentence(quote.quote.as_str(), text_size, image, &font, time_size);
    draw_sentence(quote.solution.as_str(), text_size, image, &font, time_size + text_size);
}

fn draw_time(text: &str, font_size: u32, image: &mut ImageBuffer<Rgb<u8>, Vec<u8>>, font: &Font, offset: u32) {
    let small_scale = Scale { x: font_size as f32, y: font_size as f32 };
    draw_text_mut(image, Rgb([0u8, 0u8, 0u8]), 0, offset, small_scale, &font, text);
}

fn draw_sentence(text: &str, font_size: u32, image: &mut ImageBuffer<Rgb<u8>, Vec<u8>>, font: &Font, offset: u32) {
    let scale = Scale { x: font_size as f32, y: font_size as f32 };
    let split = text.split("\n");

    let mut line = 0;
    let mut current_pos = Position {
        x: 0,
        y: offset,
    };
    for text in split {
        for char in text.chars() {
            let (w, _h) = text_size(scale, font, char.to_string().as_str());
            if current_pos.x + w as u32 > WIDTH {
                line = line + 1;
                current_pos.y = current_pos.y + font_size;
            }

            draw_text_mut(image, Rgb([0u8, 0u8, 0u8]), current_pos.x, current_pos.y, scale, &font, char.to_string().as_str());

            current_pos.x = current_pos.x + w as u32;
        }

        line = line + 1;
        current_pos.x = 0;
        current_pos.y = current_pos.y + font_size;
    }
}

fn read_font() -> Font<'static> {
    let font = Vec::from(FONT_BYTES);
    let font = Font::try_from_vec(font).unwrap();
    font
}

fn time_now() -> String {
    let utc: DateTime<Local> = Local::now();
    let delayed_format = utc.format("%Y-%m-%d %H:%M:%S");

    format!("updated time: {}", delayed_format.to_string())
}
