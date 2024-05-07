use wasm_bindgen::prelude::wasm_bindgen;

mod board;
mod tile;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn debug(s: &str);

    #[wasm_bindgen(js_namespace = Math)]
    ///
    /// Returns a random number that's greater than or equal to 0 and less than 1.
    ///
    pub fn random() -> f64;
}

///
/// The directions that a tile can be adjacent to.
///
pub const DIERECTIONS: [(i32, i32); 8] = [
    (1, 1),
    (1, 0),
    (1, -1),
    (-1, 1),
    (-1, 0),
    (-1, -1),
    (0, 1),
    (0, -1),
];

#[wasm_bindgen]
#[derive(PartialEq, Clone, Copy)]
pub enum GameState {
    Ready,
    Playing,
    Lost,
    Won,
}

#[wasm_bindgen]
#[derive(PartialEq, Eq, Hash)]
pub enum Difficulty {
    Easy,
    Medium,
    Hard,
}

impl Difficulty {
    ///
    /// Returns the configuration for the difficulty
    /// which is the width, height, and number of mines respectively.
    ///
    pub fn get_config(&self) -> (usize, usize, usize) {
        match self {
            Difficulty::Easy => (9, 9, 10),
            Difficulty::Medium => (16, 16, 40),
            Difficulty::Hard => (30, 16, 99),
        }
    }
}
