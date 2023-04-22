use linked_list::{Cursor, LinkedList};
use sdl2::image::LoadTexture;
use sdl2::{render::WindowCanvas, video::FullscreenType, Sdl};
use std::{ffi::OsString, path::PathBuf};

/// Navigator holds surface state
pub struct Navigator<'a> {
    cursor: Cursor<'a, PathBuf>,
    image: PathBuf,
    rotation: f64,
    fullscreen: FullscreenType,
    pub pageant_mode: bool,
    pub pageant_ready: bool,
    canvas: WindowCanvas,
    window_title: OsString,
}

impl<'a> Navigator<'a> {
    pub fn new(list: &'a mut LinkedList<PathBuf>, sdl_context: Sdl) -> Result<Self, String> {
        let mut cursor = list.cursor();
        let image = cursor.next().expect("No images found").to_owned();
        let window_title = image.file_name().unwrap().to_owned();
        let fullscreen = FullscreenType::Off;
        let rotation: f64 = 0.0;
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
        let texture = texture_creator.load_texture(self.image.clone())?;
        self.canvas.copy_ex(
            &texture,
            None,
            None,
            self.rotation * -90_f64,
            None,
            false,
            false,
        )?;
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
    /// peek ahead to see if the next image can be loaded onto a texture
    /// if not, remove it from the list and recur
    pub fn next(&mut self) -> Result<(), String> {
        // if there is a next item in list assign it to image
        // otherwise must be at the end. So reset and peek again
        let image = if let Some(image) = self.cursor.peek_next() {
            image.to_owned()
        } else {
            // return to beginning
            self.cursor.reset();
            if let Some(image) = self.cursor.peek_next() {
                image.to_path_buf()
            } else {
                return Err("no images found".to_string());
            }
        };

        // if `image` can be loaded onto a texture it is supported
        // otherwise remove it from the list
        let texture_creator = self.canvas.texture_creator();
        if let Ok(_t) = texture_creator.load_texture(image) {
            // we could load the image in the texture, so let's move the cursor
            self.image = self.cursor.next().unwrap().to_path_buf();
            // this unwrap is safe as long as we filter out directories
            // in the get_files() utility
            self.window_title = self.image.file_name().unwrap().to_owned();
            self.update_canvas()?;
            self.update_window()?;
        } else {
            self.cursor.remove();
            // try again
            self.next()?;
        };
        Ok(())
    }
    /// prev() works differently from next() b/c there is no
    /// remove_prev(). So we go a head and advance (reverse?) the cursor
    /// so that when we call reverse it is in the right place
    pub fn prev(&mut self) -> Result<(), String> {
        // if there is no previous we must be at the beginning
        // so call prev() again
        self.image = if let Some(image) = self.cursor.prev() {
            image.to_owned()
        } else if let Some(image) = self.cursor.prev() {
            image.to_path_buf()
        } else {
            return Err("no images found".to_string());
        };
        // if `image` can be loaded onto a texture it is supported
        // otherwise remove it from the list
        let texture_creator = self.canvas.texture_creator();
        if let Ok(_t) = texture_creator.load_texture(self.image.clone()) {
            // this unwrap is safe as long as we filter out directories
            // in the get_files() utility
            self.window_title = self.image.file_name().unwrap().to_owned();
            self.update_canvas()?;
            self.update_window()?;
        } else {
            // file cannot be loaded into a texture, so we don't want it
            self.cursor.remove();
            self.prev()?;
        };
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
}
