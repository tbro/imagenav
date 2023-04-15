use raw_tty::IntoRawMode;
use rayon::prelude::*;
use sdl2::video::FullscreenType;
use std::io::{stdin, Read};
use std::ops::Deref;
use std::path::{Path, PathBuf};
use std::sync::mpsc::{self, Receiver};
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};
use terminal_keycode::{Decoder, KeyCode};

/// Build a vec of files in the given directory
/// filtering out random filesystem detritus. We use
/// Rayon's multi-threaded iterators so it's not so slow.
pub fn get_paths(path: &Path) -> Result<Vec<PathBuf>, String> {
    let read_dir = std::fs::read_dir(path).map_err(|e| e.to_string())?;
    let mut files = read_dir
        .into_iter()
        .par_bridge()
        // filter out i/o errors
        .filter_map(|x| x.ok())
        .map(|x| x.path())
        // filter out directories
        .filter(|x| x.file_name().is_some())
        .collect::<Vec<PathBuf>>();

    if files.is_empty() {
        return Err("no files found in image directory".to_string());
    }
    files.par_sort_unstable_by(|a, b| a.file_name().unwrap().cmp(b.file_name().unwrap()));
    Ok(files)
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
