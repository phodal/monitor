use std::path::Path;
use std::thread::sleep;
use std::time::Duration;

use chrono::prelude::*;
use image::{ImageBuffer, Rgb};
use reqwest::Client;
use rusttype::{Font};
use serde::{Deserialize, Serialize};
use crate::monitor_canvas::MonitorCanvas;

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
const WIDTH: u32 = 1200;
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
    MonitorCanvas::draw_english(time.as_str(), time_size, image, &font, 0);

    let text_size = 80;
    MonitorCanvas::draw_chinese(quote.quote.as_str(), text_size, image, &font, time_size);
    MonitorCanvas::draw_chinese(quote.solution.as_str(), text_size, image, &font, time_size + text_size);
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
