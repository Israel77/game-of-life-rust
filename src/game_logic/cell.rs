use serde::{Serialize, Deserialize};

#[derive(Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum CellState {
    Alive,
    Dead
}

#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct Cell {
    state: CellState,
}

impl Cell {

    pub fn new(state: CellState) -> Cell {
        Cell {
            state: state
        }
    }

    pub fn update(&mut self, number_of_neighbors: u8) {
        match self.state {
            CellState::Alive => if (number_of_neighbors < 2) || (number_of_neighbors > 3) {
                self.state = CellState::Dead
            }
            CellState::Dead => if number_of_neighbors == 3 {
                self.state = CellState::Alive;
            }
        }
    }

    pub fn get_state(&self) -> &CellState {
        &self.state
    }
}

