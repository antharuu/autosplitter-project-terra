use std::{thread, time::Duration};
use std::path::Path;
use std::time::Instant;

use enigo::*;
use notify::{RecursiveMode, Result, Watcher};

use crate::config::{RESET_KEY, SPLIT_KEY};

mod levels;
mod config;

const BASE_SAVE_CONTENT: &str = "46A1W1KiFmKNnUCor9PzKA==";

fn press_key(enigo: &mut Enigo, key: Key) {
    enigo.key_down(key);
    thread::sleep(Duration::from_millis(100));
    enigo.key_up(key);
}

fn main() -> Result<()> {
    let mut save_path = dirs_next::data_local_dir().unwrap(); // Obtient le chemin vers LocalLow
    save_path.push("..\\LocalLow\\Helios Project\\Project TERRA\\Save1.sav");

    let _levels = levels::get_levels();

    let mut enigo = Enigo::new();
    let mut save_content: String = BASE_SAVE_CONTENT.to_string();

    let save_path_clone = save_path.clone(); // Clone le chemin pour la fermeture

    let mut last_execution_time = Instant::now(); // Ajout pour suivre le temps

    println!("Watching save file at: {:?}", save_path);

    let mut watcher = notify::recommended_watcher(move |res: Result<_>| {
        let save_path = save_path_clone.clone(); // Re-clone à chaque appel de la fermeture
        match res {
            Ok(_event) => {
                // Vérifie si au moins 2 secondes se sont écoulées
                if last_execution_time.elapsed() >= Duration::from_secs(2) {
                    println!("---------------------------------");
                    match std::fs::read_to_string(&save_path) { // Utilisez une référence ici
                        Ok(content) => {
                            let is_same_as_last = content == save_content;
                            let is_base_content = content == BASE_SAVE_CONTENT;

                            if is_base_content && !is_same_as_last {
                                println!("Resetting save file...");
                                press_key(&mut enigo, RESET_KEY);
                            } else {
                                press_key(&mut enigo, SPLIT_KEY);
                            }

                            save_content = content;
                            last_execution_time = Instant::now();

                            println!("Splitting...");
                        }
                        Err(e) => {
                            println!("Erreur lors de la lecture du fichier : {}", e);
                        }
                    }
                }
            }
            Err(e) => println!("Erreur de surveillance : {:?}", e),
        }
    })?;

    watcher.watch(Path::new(&save_path), RecursiveMode::Recursive)?;

    loop {
        thread::sleep(Duration::from_millis(250));
    }
}
