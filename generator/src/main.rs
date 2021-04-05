use image::{Rgb};
use imageproc::drawing::{draw_text_mut, text_size};
use rusttype::{Font, Scale};
use std::path::Path;
use image::ImageBuffer;

use serde::{Serialize, Deserialize};
use reqwest::Client;

#[derive(Deserialize, Serialize, Debug)]
struct TaskList {
    value: Vec<Item>,
    #[serde(flatten, rename = "@odata.context")]
    data_context: String,
}


#[derive(Deserialize, Serialize, Debug)]
struct Item {
    #[serde(flatten, rename = "@odata.etag")]
    data_etag: String,
    importance: String,
    #[serde(flatten, rename = "isReminderOn")]
    is_reminder_on: bool,
    status: bool,
    title: String,
    #[serde(flatten, rename = "createdDateTime")]
    created_date_time: String,
    #[serde(flatten, rename = "lastModifiedDateTime")]
    last_modified_date_time: String,
    id: String,
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let request_url = format!("https://graph.microsoft.com/v1.0/me/todo/lists/{id}/tasks",
                              id = "AQMkADAwATM0MDAAMS04N2E2LWRhMDItMDACLTAwCgAuAAADlaHHnFnn1UuoFE2pMt0j5QEAaMraZPkN_0mltv0IMNqe5wAD5QRmOQAAAA==");
    println!("{}", request_url);
    let client = Client::new();
    let response =
        client.get(&request_url)
            .bearer_auth("")
            .send()
            .await?;

    println!("{:?}", response.text().await?);
    // let users: Vec<TaskList> = response.json().await?;
    // println!("{:?}", users);
    write_text();

    Ok(())
}

fn write_text() {
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
