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
    pub pageant_mode: bool,
    pub pageant_ready: bool,
    pub canvas: WindowCanvas,
    window_title: OsString,
}

impl<'a> Navigator<'a> {
    pub fn new(list: &'a mut LinkedList<PathBuf>, sdl_context: Sdl) -> Result<Self, String> {
        let mut cursor = list.cursor();
        let image = cursor.next().expect("No images found").to_owned();
        let window_title = image.file_name().unwrap().to_owned();
        let fullscreen = FullscreenType::Off;
        let rotation: f64 = 0.0;
        let zoom: f64 = 1.0;
        let pageant_mode = false;
        let pageant_ready = false;
        let video_subsystem = sdl_context.video()?;
        let window = video_subsystem
            .window("imagenav", 800, 600)
            .resizable()
            .build()
            .map_err(|e| e.to_string())
            .unwrap();

        let canvas = window
            .into_canvas()
            .present_vsync()
            .accelerated()
            .build()
            .map_err(|e| e.to_string())?;
	// println!("{:?}", canvas.info());
        let s = Self {
            cursor,
            fullscreen,
            rotation,
            zoom,
            image,
            pageant_mode,
            canvas,
            window_title,
            pageant_ready,
        };
        Ok(s)
    }
    fn update_canvas(&mut self) -> Result<(), String> {
        self.canvas.clear();
        let texture_creator = self.canvas.texture_creator();
        let texture = texture_creator
            .load_texture(self.image.clone())
            .map_err(|e| e.to_string())?;
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
        Ok(())
    }
    fn update_window(&mut self) -> Result<(), String> {
        let window = self.canvas.window_mut();
        window.set_fullscreen(self.fullscreen).unwrap();
        window
            .set_title(self.window_title.to_str().unwrap())
            .map_err(|e| e.to_string())?;
        Ok(())
    }
    pub fn next(&mut self) -> Result<(), String> {
        self.image = if let Some(image) = self.cursor.next() {
            image.to_owned()
        } else {
            self.cursor.reset();
            let next = self.cursor.next().unwrap();
            next.to_owned()
        };
        self.window_title = self.image.file_name().unwrap().to_owned();
        self.update_canvas()?;
        self.update_window()?;
        Ok(())
    }
    pub fn prev(&mut self) -> Result<(), String> {
        self.image = if let Some(image) = self.cursor.prev() {
            image.to_owned()
        } else {
            self.cursor.prev().unwrap().to_owned()
        };
        self.window_title = self.image.file_name().unwrap().to_owned();
        self.update_canvas()?;
        self.update_window()?;
        Ok(())
    }
    pub fn fullscreen_toggle(&mut self) -> Result<(), String> {
        match self.fullscreen {
            FullscreenType::Off => self.fullscreen = FullscreenType::Desktop,
            FullscreenType::True => self.fullscreen = FullscreenType::Off,
            FullscreenType::Desktop => self.fullscreen = FullscreenType::Off,
        };

        let window = self.canvas.window_mut();
        window.set_fullscreen(self.fullscreen)?;
        self.canvas.present();
        Ok(())
    }
    pub fn pageant_toggle(&mut self) {
        self.pageant_mode = !self.pageant_mode;
    }
    pub fn rotate(&mut self, f: f64) -> Result<(), String> {
        self.rotation += f;
        self.update_canvas()?;
        Ok(())
    }
    pub fn zoom(&mut self, f: f64) {
        self.zoom += f;
    }
}
