use image::{ImageBuffer, Rgb};
use rusttype::{Font, Scale};
use imageproc::drawing::{draw_text_mut, text_size};
use crate::Position;
use regex::Regex;

pub struct MonitorCanvas<'i> {
    pub image: &'i mut ImageBuffer<Rgb<u8>, Vec<u8>>,
    pub font: &'i Font<'i>,
    pub width: u32,
    pub height: u32,
}

impl<'i> MonitorCanvas<'i> {
    pub fn new(width: u32, height: u32, font: &'i Font, image: &'i mut ImageBuffer<Rgb<u8>, Vec<u8>>) -> MonitorCanvas<'i> {
        MonitorCanvas {
            image,
            font,
            width,
            height,
        }
    }

    pub fn draw_english(&mut self, text: &str, font_size: u32, offset: u32) {
        let small_scale = Scale { x: font_size as f32, y: font_size as f32 };
        draw_text_mut(self.image, Rgb([0u8, 0u8, 0u8]), 0, offset, small_scale, self.font, text);
    }

    pub fn is_need_space(str: &str) -> bool {
        let space_char = Regex::new("[，。！《》『』？、,.?\"]").unwrap();
        space_char.is_match(str)
    }

    pub fn draw_chinese(&mut self, text: &str, font_size: u32, offset: u32) {
        let scale = Scale { x: font_size as f32, y: font_size as f32 };
        let split = text.split("\n");

        let mut line = 0;
        let mut current_pos = Position {
            x: 0,
            y: offset,
        };
        for text in split {
            for char in text.chars() {
                let (w, _h) = text_size(scale, self.font, char.to_string().as_str());
                if current_pos.x + w as u32 > self.width {
                    line = line + 1;
                    current_pos.x = 0;
                    current_pos.y = current_pos.y + font_size;
                }

                draw_text_mut(self.image, Rgb([0u8, 0u8, 0u8]), current_pos.x, current_pos.y, scale, self.font, char.to_string().as_str());
                if w == 0 {
                    current_pos.x = current_pos.x + font_size / 3;
                } else if MonitorCanvas::is_need_space(char.to_string().as_str()) {
                    current_pos.x = current_pos.x + font_size * 2 / 3;
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


#[cfg(test)]
mod tests {
    use crate::monitor_canvas::MonitorCanvas;

    #[test]
    fn should_handle_for_chinese_chars() {
        assert!(MonitorCanvas::is_need_space("，"));
        assert!(MonitorCanvas::is_need_space(","));
        assert!(MonitorCanvas::is_need_space("。"));
        assert!(MonitorCanvas::is_need_space("."));
    }
}