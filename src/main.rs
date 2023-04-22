use linked_list::LinkedList;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
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
    let files = get_paths(path)?;
    let mut list = LinkedList::new();
    for path in files {
        list.push_back(path);
    }

    let sdl_context = sdl2::init()?;
    let mut event_pump = sdl_context.event_pump()?;
    // we set a timer to use for pageant mode
    let timer = sdl_context.timer()?;
    // initialize navigator
    let mut nav = Navigator::new(&mut list, sdl_context)?;

    // show first image
    nav.next()?;

    // track if we are exiting
    let should_exit = Arc::new(Mutex::new(false));

    let (stdin_channel, handle) = spawn_stdin_channel(should_exit.clone());
    'running: loop {
        if *should_exit.lock().unwrap() {
            break 'running;
        }
        // sdtin events
        match stdin_channel.try_recv() {
            Ok(keycode) => {
                match keycode {
                    KeyCode::Char('q') | KeyCode::CtrlC => *should_exit.lock().unwrap() = true,
                    KeyCode::Char('f') => nav.fullscreen_toggle()?,
                    KeyCode::Char('r') => nav.rotate(1.0)?,
                    KeyCode::Char('p') | KeyCode::Space => nav.pageant_toggle(),
                    KeyCode::ArrowRight => nav.next()?,
                    KeyCode::ArrowLeft => nav.prev()?,
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

        // register window events to make OS happy
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape) | Some(Keycode::Q),
                    ..
                } => *should_exit.lock().unwrap() = true,
                _ => {}
            };
        }

        let ticks = timer.ticks() as i32;
        // call next every second
        if nav.pageant_mode && (ticks / 100) % 10 == 0 && nav.pageant_ready {
            nav.next()?;
            nav.pageant_ready = false;
        } else if nav.pageant_mode && (ticks / 100) % 10 != 0 && !nav.pageant_ready {
            nav.pageant_ready = true;
        }
        sleep(10);
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
