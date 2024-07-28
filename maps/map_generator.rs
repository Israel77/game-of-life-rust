use bincode;
use std::env;
use std::fs::File;
use std::path::Path;

use game_of_life::game_logic::game::Game;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file = Path::new(&args[1]);
    println!("{:?}", file);

    let target_dir = Path::new(&args[2]);
    if file.is_file(){
        let game = Game::from_file(file);
        let target = File::create(format!(
            "{}/{}",
            target_dir,
            file.file_name().unwrap().to_str().unwrap()
        ))
        .unwrap();
        if let Ok(_) = bincode::serialize_into(target, &game) {
            println!("{:?} Created successfully", file.file_name());
        }
    };

    // if let Ok(entries) = fs::read_dir("maps/raw_maps") {
    //     for entry in entries {
    //         if let Ok(entry) = entry {
    //             // Here, `entry` is a `DirEntry`.
    //             println!("{:?}", entry.file_name());

    //             let game = Game::from_file(entry.path());
    //             let target = File::create(format!("maps/compiled/{:?}", entry.file_name())).unwrap();
    //             if let Ok(_) = bincode::serialize_into(target, &game) {
    //                 println!("{:?} Created successfully", entry.file_name());
    //             };
    //         }
    //     }
    // }
}
