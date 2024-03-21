use std::{thread, time::{Duration, Instant}};
use std::fs;

use dirs_next::data_local_dir;
use enigo::{Enigo, Key, KeyboardControllable};
use notify::{RecursiveMode, Result, recommended_watcher, Watcher};

use crate::config::{RESET_KEY, SPLIT_KEY};

mod config;
mod levels;

const BASE_SAVE_CONTENT: &str = "46A1W1KiFmKNnUCor9PzKA==";

fn press_key(enigo: &mut Enigo, key: Key) {
    enigo.key_down(key);
    thread::sleep(Duration::from_millis(100));
    enigo.key_up(key);
}

fn main() -> Result<()> {
    let save_path = setup_save_path();
    let mut enigo = Enigo::new();

    println!("Watching save file at: {:?}", save_path);

    let mut state = WatcherState::new(enigo, BASE_SAVE_CONTENT.to_string());

    let mut watcher = recommended_watcher(move |res| state.handle_file_event(res))?;
    watcher.watch((&save_path).as_ref(), RecursiveMode::Recursive)?;

    loop {
        thread::sleep(Duration::from_millis(250));
    }
}

fn setup_save_path() -> String {
    let mut save_path = data_local_dir().expect("Failed to find local data directory");
    save_path.push("..\\LocalLow\\Helios Project\\Project TERRA\\Save1.sav");
    save_path.to_str().unwrap().to_owned()
}

fn split(enigo: &mut Enigo) {
    println!("Splitting...");
    press_key(enigo, SPLIT_KEY);
}

fn reset(enigo: &mut Enigo) {
    println!("Resetting...");
    press_key(enigo, RESET_KEY);
}

struct WatcherState {
    enigo: Enigo,
    save_content: String,
    last_execution_time: Instant,
}

impl WatcherState {
    fn new(enigo: Enigo, save_content: String) -> Self {
        Self {
            enigo,
            save_content,
            last_execution_time: Instant::now(),
        }
    }

    fn handle_file_event(&mut self, res: Result<notify::Event>) {
        let save_path = setup_save_path(); // Adjust according to your path logic
        match res {
            Ok(_event) => {
                if self.last_execution_time.elapsed() >= Duration::from_secs(2) {
                    match fs::read_to_string(&save_path) {
                        Ok(content) => self.process_content_change(&content),
                        Err(e) => println!("Error reading file: {}", e),
                    }
                }
            }
            Err(e) => println!("Watch error: {:?}", e),
        }
    }

    fn process_content_change(&mut self, content: &str) {
        let is_same_as_last = content == self.save_content;
        let is_base_content = content == BASE_SAVE_CONTENT;

        if is_base_content && !is_same_as_last {
            reset(&mut self.enigo);
        } else if !is_same_as_last {
            println!("Base:\t\t{}", BASE_SAVE_CONTENT);
            println!("Current:\t{}", content);
            println!("Last:\t\t{}", self.save_content);

            split(&mut self.enigo);
        }

        self.save_content = content.to_string();
        self.last_execution_time = Instant::now();
    }
}
