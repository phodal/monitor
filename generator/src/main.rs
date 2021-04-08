use std::path::Path;

use chrono::prelude::*;
use image::{ImageBuffer, Rgb};
use reqwest::Client;
use rusttype::{Font};
use serde::{Deserialize, Serialize};
use crate::monitor_canvas::MonitorCanvas;
use crate::monitor::Monitor;

pub mod monitor_canvas;
pub mod monitor;

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
        let data: Quote = response.json().await?;

        let mut image = ImageBuffer::from_pixel(WIDTH, HEIGHT, Rgb([255, 255, 255]));
        draw_content(data, &mut image);
        let _ = image.save(Path::new("monitor.bmp")).unwrap();

        Monitor::display();
        Monitor::m_sleep();
    }
}

fn draw_content(quote: Quote, image: &mut ImageBuffer<Rgb<u8>, Vec<u8>>) {
    let font = read_font();

    let time = time_now();
    let time_size = 40;

    let mut canvas = MonitorCanvas::new(WIDTH, HEIGHT, &font, image);
    canvas.draw_english(time.as_str(), time_size, 0);

    let text_size = 60;
    canvas.draw_chinese(quote.quote.as_str(), text_size, time_size);
    canvas.draw_chinese(quote.solution.as_str(), text_size, time_size + text_size);
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
