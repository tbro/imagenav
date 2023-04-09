use linked_list::{Cursor, LinkedList};
use sdl2::image::LoadTexture;
use sdl2::{render::WindowCanvas, video::FullscreenType, Sdl};
use std::{ffi::OsString, path::PathBuf};

/// Navigator holds surface state
pub struct Navigator<'a> {
    pub cursor: Cursor<'a, PathBuf>,
    pub image: PathBuf,
    pub rotation: f64,
    pub zoom: f64,
    pub fullscreen: FullscreenType,
    pub progress: bool,
    pub progress_timer: u16,
    pub canvas: WindowCanvas,
    window_title: OsString,
}

impl<'a> Navigator<'a> {
    pub fn new(list: &'a mut LinkedList<PathBuf>, sdl_context: Sdl) -> Self {
        let mut cursor = list.cursor();
        let image = cursor.next().expect("No images found").to_owned();
        let window_title = image.file_name().unwrap().to_owned(); //.to_str().unwrap();
        let fullscreen = FullscreenType::Off;
        let rotation: f64 = 0.0;
        let zoom: f64 = 1.0;
        let progress = false;
        let progress_timer = 0;
        let video_subsystem = sdl_context.video().unwrap();
        let window = video_subsystem
            .window("imagenav", 800, 600)
            .resizable()
            .build()
            .map_err(|e| e.to_string())
            .unwrap();

        let canvas = window
            .into_canvas()
            .present_vsync() // or .software()
            .build()
            .map_err(|e| e.to_string())
            .unwrap();

        Self {
            cursor,
            fullscreen,
            rotation,
            zoom,
            image,
            progress,
            progress_timer,
            canvas,
            window_title,
        }
    }
    fn update_canvas(&mut self) {
	self.canvas.clear();
        let texture_creator = self.canvas.texture_creator();
        let texture = texture_creator.load_texture(self.image.clone()).unwrap();
        self.canvas
            .copy_ex(
                &texture,
                None,
                None,
                self.rotation * -90_f64,
                None,
                false,
                false,
            )
            .unwrap();
        self.canvas.present();
    }
    fn update_window(&mut self) {
        let window = self.canvas.window_mut();
        window.set_fullscreen(self.fullscreen).unwrap();
        window
            .set_title(self.window_title.to_str().unwrap())
            .map_err(|e| e.to_string())
            .unwrap();
    }
    pub fn next(&mut self) {
        self.image = if let Some(image) = self.cursor.next() {
            image.to_owned()
        } else {
            self.cursor.reset();
            self.cursor.next().unwrap().to_owned()
        };
        self.window_title = self.image.file_name().unwrap().to_owned();
        self.update_canvas();
        self.update_window();
    }
    pub fn prev(&mut self) {
        self.image = if let Some(image) = self.cursor.prev() {
            image.to_owned()
        } else {
            self.cursor.prev().unwrap().to_owned()
        };
        self.window_title = self.image.file_name().unwrap().to_owned();
        self.update_canvas();
        self.update_window();
    }
    pub fn fullscreen_toggle(&mut self) {
        match self.fullscreen {
            FullscreenType::Off => self.fullscreen = FullscreenType::Desktop,
            FullscreenType::True => self.fullscreen = FullscreenType::Off,
            FullscreenType::Desktop => self.fullscreen = FullscreenType::Off,
        };

        let window = self.canvas.window_mut();
        window.set_fullscreen(self.fullscreen).unwrap();
        self.canvas.present();
    }
    pub fn presentation_toggle(&mut self) {
        self.progress = !self.progress;
    }
    pub fn rotate(&mut self, f: f64) {
        self.rotation += f;
        self.update_canvas();
    }
    pub fn zoom(&mut self, f: f64) {
        self.zoom += f;
    }
}
