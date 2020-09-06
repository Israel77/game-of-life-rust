use bincode;
use std::fs;
use std::fs::{File};

use game_of_life::game_logic::game::Game;

fn main() {
    if let Ok(entries) = fs::read_dir("maps/raw_maps") {
        for entry in entries {
            if let Ok(entry) = entry {
                // Here, `entry` is a `DirEntry`.
                println!("{:?}", entry.file_name());

                let game = Game::from_file(entry.path());
                let target = File::create(format!("maps/compiled/{:?}", entry.file_name())).unwrap();
                if let Ok(_) = bincode::serialize_into(target, &game) {
                    println!("{:?} Created successfully", entry.file_name());
                };
            }
        }
    }
}
