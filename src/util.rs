use raw_tty::IntoRawMode;
use sdl2::image::LoadSurface;
use sdl2::surface::Surface;
use sdl2::video::FullscreenType;
use std::io::{stdin, Read};
use std::ops::Deref;
use std::path::{Path, PathBuf};
use std::sync::mpsc::{self, Receiver};
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};
use terminal_keycode::{Decoder, KeyCode};

pub fn get_paths(path: &Path) -> Vec<PathBuf> {
    let read_dir = std::fs::read_dir(path).unwrap();
    let mut vec = read_dir
        .into_iter()
        .filter_map(|x| x.ok())
        // filter out random fs detritus
        .filter(|x| Surface::from_file(x.path()).is_ok())
        .map(|x| x.path())
        .collect::<Vec<PathBuf>>();

    vec.sort_by(|a,b|a.file_name().unwrap().cmp(b.file_name().unwrap()));
    vec
}

#[derive(Copy, Clone, Debug)]
pub struct MyFullscreenType(pub FullscreenType);

impl MyFullscreenType {
    pub fn toggle(&mut self) {
        match self.0 {
            FullscreenType::Off => self.0 = FullscreenType::Desktop,
            FullscreenType::True => self.0 = FullscreenType::Off,
            FullscreenType::Desktop => self.0 = FullscreenType::Off,
        };
    }
}

impl Deref for MyFullscreenType {
    type Target = FullscreenType;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub fn spawn_stdin_channel(should_exit: Arc<Mutex<bool>>) -> (Receiver<KeyCode>, JoinHandle<()>) {
    let (tx, rx) = mpsc::channel::<KeyCode>();

    // let guard = std::io::stdin().guard_mode().unwrap();
    let mut stdin = stdin().into_raw_mode().unwrap();
    let mut decoder = Decoder::new();
    let handle = thread::spawn(move || loop {
        if *should_exit.lock().unwrap() {
            break;
        }
        let mut buf = vec![0];
        stdin.read_exact(&mut buf).unwrap();

        for keycode in decoder.write(buf[0]) {
            tx.send(keycode).unwrap();
        }
    });
    (rx, handle)
}
