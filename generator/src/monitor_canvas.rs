use image::{ImageBuffer, Rgb};
use rusttype::{Font, Scale};
use imageproc::drawing::{draw_text_mut, text_size};
use crate::Position;

const WIDTH: u32 = 1200;

pub struct MonitorCanvas<'i> {
    pub image: &'i mut ImageBuffer<Rgb<u8>, Vec<u8>>,
    pub font: &'i Font<'i>,
    pub width: u32,
    pub height: u32,
}

impl<'i> MonitorCanvas<'i> {
    pub fn new() {

    }

    pub fn draw_english(text: &str, font_size: u32, image: &mut ImageBuffer<Rgb<u8>, Vec<u8>>, font: &Font, offset: u32) {
        let small_scale = Scale { x: font_size as f32, y: font_size as f32 };
        draw_text_mut(image, Rgb([0u8, 0u8, 0u8]), 0, offset, small_scale, &font, text);
    }

    pub fn draw_chinese(text: &str, font_size: u32, image: &mut ImageBuffer<Rgb<u8>, Vec<u8>>, font: &Font, offset: u32) {
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
                if w < (font_size / 3 * 2) as i32 {
                    current_pos.x = current_pos.x + font_size / 3 * 2;
                } else {
                    current_pos.x = current_pos.x + w as u32;
                }
            }

            line = line + 1;
            current_pos.x = 0;
            current_pos.y = current_pos.y + font_size;
        }
    }
}

