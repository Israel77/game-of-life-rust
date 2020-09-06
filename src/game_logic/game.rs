use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;


use bincode;
use regex::Regex;

use super::cell::Cell;
use super::cell::CellState;

pub type GameState = Vec<Vec<Cell>>;

#[derive(Clone, Serialize, Deserialize)]
pub struct Game {
    pub state: GameState,
    pub rows: u16,
    pub cols: u16,
}

impl Game {
    // pub fn new(rows: u16, cols: u16) -> Game {
    //     Game {
    //         state: vec![Cell::new(CellState::Dead); (rows * cols) as usize],
    //         rows: rows,
    //         cols: cols,
    //     }
    // }

    pub fn from_string(src: &str, alive: char, dead: char, size: (u16, u16)) -> Game {
        let (rows, cols) = size;
        let mut src_as_chars = src.chars();
        let mut state = vec![];
        for _ in 0..rows {
            let mut row = vec![];
            let mut _counter = 0;
            for _ in 0..cols {
                let c = src_as_chars.nth(0).unwrap();
                match c {
                    a if a == alive => row.push(Cell::new(CellState::Alive)),
                    a if a == dead => row.push(Cell::new(CellState::Dead)),
                    _ => (),
                }
                _counter += 1;
            }
            if size.1 == row.len() as u16 {
                state.push(row);
            } else {
                panic!("Row size didn't match signature");
            }
        }

        Game {
            state: state,
            rows: size.0,
            cols: size.1,
        }
    }

    pub fn from_file<P>(filename: P) -> Game
    where
        P: AsRef<Path>,
    {
        fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
        where
            P: AsRef<Path>,
        {
            let file = File::open(filename)?;
            Ok(io::BufReader::new(file).lines())
        }

        let mut src = String::new();

        let mut lines = read_lines(filename).unwrap().map(|x| x.unwrap());
        let first_line = lines.next().unwrap();

        lazy_static! {
            static ref HEADER: Regex = Regex::new(r"\((\d+), (\d+)\) +(.) +(.)").unwrap();
        };

        let header_cap = HEADER.captures(&first_line).unwrap();

        let rows: u16 = (&header_cap[1])
            .parse()
            .expect("Could not parse row number as u8");
        let cols: u16 = (&header_cap[2])
            .parse()
            .expect("Could not parse row number as u8");
        let alive: char = (&header_cap[3]).chars().next().unwrap();
        let dead: char = (&header_cap[4]).chars().next().unwrap();

        let raw_line_pattern = { format!("[{}{}]{{{}}}", alive, dead, cols).replace(".", "\\.") };
        let line_pattern: Regex = Regex::new(&raw_line_pattern).unwrap();

        for (index, line) in lines.enumerate() {
            if line_pattern.is_match(&line) && (line.len() as u16 == cols) {
                src.push_str(&line);
            } else {
                panic!("Line {} doesn't match pattern specified in header", index);
            }
        }

        Game::from_string(&src, alive, dead, (rows, cols))
    }

    pub fn from_compiled<P>(filename: P) -> Game
    where
        P: AsRef<Path>,
    {

        let reader = File::open(filename).unwrap();

        bincode::deserialize_from(reader).unwrap()
    }

    /// Update the game (1 step) according to the rules
    pub fn update(&mut self) -> () {
        let mut _state = self.state.clone();
        for i in 0..self.rows {
            for j in 0..self.cols {
                let neighbors = { self.count_neighbors(i, j) };
                _state[i as usize][j as usize].update(neighbors);
            }
        }
        self.state = _state;
    }

    pub fn get_item(&self, i: u16, j: u16) -> &Cell {
        &(self.state[i as usize][j as usize])
    }

    fn count_neighbors(&self, row: u16, col: u16) -> u8 {
        fn cell_to_int(cell: &Cell) -> u8 {
            match cell.get_state() {
                CellState::Alive => 1,
                CellState::Dead => 0,
            }
        }

        let max_row = self.rows - 1;
        let max_col = self.cols - 1;

        match (row, col) {
            // Top-left corner
            a if a == (0, 0) => [
                self.get_item(row + 1, col),
                self.get_item(row, col + 1),
                self.get_item(row + 1, col + 1),
            ]
            .iter()
            .fold(0, |acc: u8, c| acc + cell_to_int(c)),

            // Top-right corner
            a if a == (0, max_col) => [
                self.get_item(row, col - 1),
                self.get_item(row + 1, col - 1),
                self.get_item(row + 1, col),
            ]
            .iter()
            .fold(0, |acc: u8, c| acc + cell_to_int(c)),

            // Bottom-left corner
            a if a == (max_row, 0) => [
                self.get_item(row - 1, col),
                self.get_item(row - 1, col + 1),
                self.get_item(row, col + 1),
            ]
            .iter()
            .fold(0, |acc: u8, c| acc + cell_to_int(c)),

            // Bottom-right corner
            a if a == (max_row, max_col) => [
                self.get_item(row - 1, col - 1),
                self.get_item(row, col - 1),
                self.get_item(row - 1, col),
            ]
            .iter()
            .fold(0, |acc: u8, c| acc + cell_to_int(c)),

            // Top row
            a if a.0 == 0 => [
                self.get_item(row, col - 1),
                self.get_item(row + 1, col - 1),
                self.get_item(row + 1, col),
                self.get_item(row, col + 1),
                self.get_item(row + 1, col + 1),
            ]
            .iter()
            .fold(0, |acc: u8, c| acc + cell_to_int(c)),

            // Bottom row
            a if a.0 == max_row => [
                self.get_item(row - 1, col - 1),
                self.get_item(row, col - 1),
                self.get_item(row - 1, col),
                self.get_item(row - 1, col + 1),
                self.get_item(row, col + 1),
            ]
            .iter()
            .fold(0, |acc: u8, c| acc + cell_to_int(c)),

            // Leftmost column
            a if a.1 == 0 => [
                self.get_item(row - 1, col),
                self.get_item(row + 1, col),
                self.get_item(row - 1, col + 1),
                self.get_item(row, col + 1),
                self.get_item(row + 1, col + 1),
            ]
            .iter()
            .fold(0, |acc: u8, c| acc + cell_to_int(c)),

            // Rightmost column
            a if a.1 == max_col => [
                self.get_item(row - 1, col - 1),
                self.get_item(row, col - 1),
                self.get_item(row + 1, col - 1),
                self.get_item(row - 1, col),
                self.get_item(row + 1, col),
            ]
            .iter()
            .fold(0, |acc: u8, c| acc + cell_to_int(c)),

            // Otherwise
            _ => [
                self.get_item(row - 1, col - 1),
                self.get_item(row, col - 1),
                self.get_item(row + 1, col - 1),
                self.get_item(row - 1, col),
                self.get_item(row + 1, col),
                self.get_item(row - 1, col + 1),
                self.get_item(row, col + 1),
                self.get_item(row + 1, col + 1),
            ]
            .iter()
            .fold(0, |acc: u8, c| acc + cell_to_int(c)),
        }
    }
}
