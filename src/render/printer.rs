use crate::game_logic::cell::*;
use crate::game_logic::game::*;

pub trait GameRenderer {
    fn render(&mut self, game: &Game) -> ();
}

pub struct PrintRenderer {}

pub struct FancyPrinter {
    prev_state: GameState,
}

impl FancyPrinter {
    pub fn new(state: &GameState) -> FancyPrinter {
        FancyPrinter {
            prev_state: state.clone(),
        }
    }
}

impl GameRenderer for PrintRenderer {
    fn render(&mut self, game: &Game) {
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        let rows = game.rows;
        let cols = game.cols;
        for i in 0..rows {
            for j in 0..cols {
                match game.get_item(i, j) {
                    Cell::Alive => yellow! {"@"},
                    Cell::Dead => print! {"."},
                }
            }
            println!("");
        }
    }
}

impl GameRenderer for FancyPrinter {
    fn render(&mut self, game: &Game) {
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        let rows = game.rows;
        let cols = game.cols;
        for i in 0..rows {
            for j in 0..cols {
                let previous = self.prev_state[i as usize][j as usize];
                match game.get_item(i, j) {
                    Cell::Alive => {
                        if previous == Cell::Alive {
                            yellow! {"@"}
                        } else {
                            green! {"@"}
                        }
                    }
                    Cell::Dead => {
                        if previous == Cell::Alive {
                            red! {"x"}
                        } else {
                            print! {"."}
                        }
                    }
                }
                self.prev_state[i as usize][j as usize] = *game.get_item(i, j);
            }
            println!("");
        }
    }
}
