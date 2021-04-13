use std::path::Path;

use chrono::prelude::*;
use image::{ImageBuffer, Rgb};
use reqwest::{Client};
use rusttype::{Font};
use serde::{Deserialize, Serialize};
use crate::monitor_canvas::MonitorCanvas;
use crate::monitor::Monitor;
use std::fs::File;
use std::io::Read;
use crate::todo::{TodoResponse, TodoItem};

pub mod monitor_canvas;
pub mod monitor;
pub mod todo;

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

#[derive(Deserialize, Serialize, Debug)]
struct MonitorConfig {
    id: String,
    token: String,
}

const FONT_BYTES: &'static [u8] = include_bytes!("wqy-microhei.ttc");
const WIDTH: u32 = 1200;
const HEIGHT: u32 = 825;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let _config = read_config();

    loop {
        let todos: Vec<TodoItem> = vec![];
        // let todos = get_todo(&config).await?;
        let data = get_display_data().await?;

        let mut image = ImageBuffer::from_pixel(WIDTH, HEIGHT, Rgb([255, 255, 255]));
        draw_content(todos, data, &mut image);
        let image_name = "monitor.bmp";
        let _ = image.save(Path::new(image_name)).unwrap();

        Monitor::display(image_name);
        Monitor::m_sleep();
    }
}

#[allow(dead_code)]
async fn get_todo(config: &MonitorConfig) -> Result<Vec<TodoItem>, reqwest::Error> {
    let ms_url = format!("https://graph.microsoft.com/v1.0/me/todo/lists/{id}/tasks",
                         id = &config.id);
    let client = Client::new();
    let response =
        client.get(&ms_url)
            .bearer_auth(&config.token)
            .send()
            .await?;


    match response.error_for_status() {
        Ok(res) => {
            let todo: TodoResponse = res.json().await?;
            Ok(todo.value)
        }
        Err(err) => {
            println!("{:?}", err);
            Err(err)
        }
    }
}

async fn get_display_data() -> Result<Quote, reqwest::Error> {
    let request_url = format!("https://phodal.github.io/monitor-api/api.json");
    let client = Client::new();
    let response =
        client.get(&request_url)
            .bearer_auth("")
            .send()
            .await?;
    let data: Quote = response.json().await?;
    Ok(data)
}

fn draw_content(todos: Vec<TodoItem>, quote: Quote, image: &mut ImageBuffer<Rgb<u8>, Vec<u8>>) {
    let font = read_font();

    let time = time_now();
    let time_size = 40;
    let mut offset = 0;

    let mut canvas = MonitorCanvas::new(WIDTH, HEIGHT, &font, image);
    canvas.draw_english(time.as_str(), time_size, offset);

    offset = offset + time_size;

    let text_size = draw_todos(&todos, offset, &mut canvas);

    canvas.draw_chinese(quote.solution.as_str(), text_size, offset);
}

fn draw_todos(todos: &Vec<TodoItem>, mut offset: u32, canvas: &mut MonitorCanvas) -> u32 {
    let text_size = 60;
    for item in todos {
        let title = format!(" [ ] {}", item.title);
        canvas.draw_chinese(title.as_str(), text_size, offset);
        offset = offset + text_size;
    }

    text_size
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

fn read_config() -> MonitorConfig {
    let mut file = File::open("monitor_config.json").unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();

    let config: MonitorConfig = match serde_json::from_str(&data) {
        Ok(x) => x,
        Err(err) => {
            panic!(err)
        }
    };

    config
}


#[cfg(test)]
mod tests {
    use crate::read_config;

    #[ignore]
    #[test]
    fn should_read_config() {
        let config = read_config();
        println!("config: {:?}", config);
    }
}


