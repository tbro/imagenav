use linked_list::LinkedList;
use sdl2::event::Event;
use sdl2::gfx::rotozoom::RotozoomSurface;
use sdl2::image::LoadSurface;
use sdl2::keyboard::Keycode;
use sdl2::surface::Surface;
use std::env;
use std::path::Path;
use std::sync::mpsc::TryRecvError;
use std::sync::{Arc, Mutex};
use std::{thread, time};
use terminal_keycode::KeyCode;

use crate::navigator::Navigator;
use crate::util::{get_paths, spawn_stdin_channel};

mod navigator;
mod util;

pub fn run(path: &Path) -> Result<(), String> {
    let files = get_paths(path);
    let mut list = LinkedList::new();
    for path in files {
        list.push_back(path);
    }
    // state will live in navigator
    let mut nav = Navigator::new(&mut list);

    // track if we are exiting
    let should_exit = Arc::new(Mutex::new(false));

    // spawn a thread to handle stdin
    let (stdin_channel, handle) = spawn_stdin_channel(should_exit.clone());

    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    // let _image_context = sdl2::image::init(InitFlag::PNG | InitFlag::JPG)?;
    let window = video_subsystem
        .window("rust-sdl2 demo: Window", 800, 600)
        .resizable()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window
        .into_canvas()
        .present_vsync() // or .software()
        .build()
        .map_err(|e| e.to_string())?;
    let texture_creator = canvas.texture_creator();
    let mut event_pump = sdl_context.event_pump()?;

    // update surface on every iteration
    'running: loop {
        if *should_exit.lock().unwrap() {
            break 'running;
        }
        // sdtin events
        match stdin_channel.try_recv() {
            Ok(keycode) => {
                match keycode {
                    KeyCode::Char('q') | KeyCode::CtrlC => *should_exit.lock().unwrap() = true,
                    KeyCode::Char('f') => nav.fullscreen_toggle(),
                    KeyCode::Char('r') => nav.rotate(1.0),
                    KeyCode::ArrowRight => nav.next(),
                    KeyCode::ArrowLeft => nav.prev(),
                    _ => {
                        print![
                            "code={:?} bytes={:?} printable={:?}\r\n",
                            keycode,
                            keycode.bytes(),
                            keycode.printable()
                        ];
                    }
                };
            }
            Err(TryRecvError::Empty) => {}
            Err(TryRecvError::Disconnected) => panic!("Channel disconnected"),
        }

        // window events
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape) | Some(Keycode::Q),
                    ..
                } => *should_exit.lock().unwrap() = true,
                Event::KeyDown {
                    keycode: Some(Keycode::F),
                    ..
                } => nav.fullscreen_toggle(),
                Event::KeyDown {
                    keycode: Some(Keycode::R),
                    ..
                } => nav.rotate(1.0),
                Event::KeyDown {
                    keycode: Some(Keycode::Z),
                    ..
                } => nav.zoom(0.5),
                _ => {}
            };
        }

        // Update the window title.
        let window = canvas.window_mut();
        let _position = window.position();
        let _size = window.size();
        let image = nav.image.clone();
        let fname = image.file_name().unwrap();
        window
            .set_title(fname.to_str().unwrap())
            .map_err(|e| e.to_string())?;
        window.set_fullscreen(nav.fullscreen)?;

        let surface = Surface::from_file(image).unwrap();
        let rotated = surface.rotozoom(nav.rotation * -90.0, 1.0, false).unwrap();
        let texture = rotated.as_texture(&texture_creator).unwrap();
        canvas.copy(&texture, None, None)?;
        canvas.present();
        sleep(1);
    }

    handle.join().unwrap();

    Ok(())
}

fn main() -> Result<(), String> {
    let args: Vec<_> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: imagenav /path/to/dir")
    } else {
        run(Path::new(&args[1]))?;
    }

    Ok(())
}

fn sleep(millis: u64) {
    let duration = time::Duration::from_millis(millis);
    thread::sleep(duration);
}
