use image::{ImageBuffer, Rgb};
use rusttype::Font;

pub struct MonitorCanvas<'i> {
    pub image: &'i mut ImageBuffer<Rgb<u8>, Vec<u8>>,
    pub font: &'i Font<'i>
}

impl<'i> MonitorCanvas<'i> {
    pub fn new() {

    }
}

