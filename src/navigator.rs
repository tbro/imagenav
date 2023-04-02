use linked_list::{Cursor, LinkedList};
use sdl2::video::FullscreenType;
use std::path::PathBuf;

/// Navigator holds surface state
pub struct Navigator<'a> {
    pub cursor: Cursor<'a, PathBuf>,
    pub image: PathBuf,
    pub rotation: f64,
    pub zoom: f64,
    pub fullscreen: FullscreenType,
}

impl<'a> Navigator<'a> {
    pub fn new(list: &'a mut LinkedList<PathBuf>) -> Self {
        let mut cursor = list.cursor();
        let image = cursor.next().expect("No images found").to_owned();
        let fullscreen = FullscreenType::Off;
        let rotation: f64 = 0.0;
        let zoom: f64 = 1.0;
        Self {
            cursor,
            fullscreen,
            rotation,
            zoom,
            image,
        }
    }
    pub fn next(&mut self) {
        let image = if let Some(image) = self.cursor.next() {
            image.to_owned()
        } else {
            self.cursor.reset();
            self.cursor.next().unwrap().to_owned()
        };
        self.image = image;
    }
    pub fn prev(&mut self) {
        let image = if let Some(image) = self.cursor.prev() {
            image.to_owned()
        } else {
            self.cursor.prev().unwrap().to_owned()
        };
        self.image = image;
    }
    pub fn fullscreen_toggle(&mut self) {
        match self.fullscreen {
            FullscreenType::Off => self.fullscreen = FullscreenType::Desktop,
            FullscreenType::True => self.fullscreen = FullscreenType::Off,
            FullscreenType::Desktop => self.fullscreen = FullscreenType::Off,
        };
    }
    pub fn rotate(&mut self, f: f64) {
        self.rotation += f;
    }
    pub fn zoom(&mut self, f: f64) {
        self.zoom += f;
    }
}
