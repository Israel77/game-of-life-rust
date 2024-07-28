use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Cell {
    Alive,
    Dead,
}

impl Cell {
    pub fn update(&self, number_of_neighbors: u8) -> Self {
        match self {
            Cell::Alive if (number_of_neighbors < 2) || (number_of_neighbors > 3) => Cell::Dead,
            Cell::Dead if number_of_neighbors == 3 => Cell::Alive,
            _ => *self,
        }
    }
}
