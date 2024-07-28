use std::env;
use std::thread;
use std::time::Duration;

use game_of_life::game_logic::game::Game;
use game_of_life::render::printer::*;

fn run(game: &mut Game, update_time: Duration, show_gen: bool) {
    let mut count = 0u32;
    let mut fp = FancyPrinter::new(&game.state);
    let mut pr = PrintRenderer {};
    loop {
        pr.render(&game);
        game.update();
        // thread::sleep(update_time * 3 / 4);
        // if show_gen {
        //     println!("{}", count);
        //     count = count + 1;
        // }

        fp.render(&game);
        if show_gen {
            println!("{}", count);
        }
        thread::sleep(update_time);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let option = &args[1];
    let path = &args[2];

    let mut game;

    match option.as_ref() {
        "map" => game = Game::from_compiled(path),
        "file" => game = Game::from_file(path),
        _ => panic!("Invalid options: {}", option),
    }

    run(&mut game, Duration::from_millis(300), false);
}
