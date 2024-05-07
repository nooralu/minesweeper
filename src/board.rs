use std::collections::HashMap;

use rand::{rngs::ThreadRng, Rng};
use wasm_bindgen::prelude::*;

use crate::{debug, settings::DIERECTIONS};

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

#[wasm_bindgen]
#[derive(Clone, Copy)]
pub struct Tile {
    index: usize,
    revealed: bool,
    adjacent_mines: usize,
    mine: Option<bool>,
    flagged: Option<bool>,
}

#[wasm_bindgen]
impl Tile {
    #[wasm_bindgen(constructor)]
    pub fn new(index: usize) -> Self {
        Self {
            index,
            revealed: false,
            adjacent_mines: 0,
            mine: None,
            flagged: None,
        }
    }

    #[wasm_bindgen(js_name = hasMine)]
    pub fn has_mine(&self) -> bool {
        self.mine.is_some_and(|mine| mine)
    }

    #[wasm_bindgen(js_name = getIndex)]
    pub fn get_index(&self) -> usize {
        self.index
    }

    #[wasm_bindgen(js_name = getAdjacentMines)]
    pub fn get_adjacent_mines(&self) -> usize {
        self.adjacent_mines
    }

    #[wasm_bindgen(js_name = isRevealed)]
    pub fn is_revealed(&self) -> bool {
        self.revealed
    }

    #[wasm_bindgen(js_name = isFlagged)]
    pub fn is_flagged(&self) -> bool {
        if let Some(flagged) = self.flagged {
            flagged
        } else {
            false
        }
    }
}

#[wasm_bindgen]
pub struct Board {
    state: GameState,
    tiles: Vec<Tile>,
    rng: ThreadRng,
    first_click: bool,
    difficuty: Difficulty,
    difficulty_map: HashMap<Difficulty, (usize, usize, usize)>,
}

#[wasm_bindgen]
impl Board {
    #[wasm_bindgen(constructor)]
    pub fn new(difficuty: Difficulty) -> Self {
        let rng = rand::thread_rng();
        let mut tiles = vec![];

        let mut difficulty_map = HashMap::with_capacity(3);
        difficulty_map.insert(Difficulty::Easy, (9, 9, 10));
        difficulty_map.insert(Difficulty::Medium, (16, 16, 40));
        difficulty_map.insert(Difficulty::Hard, (16, 30, 99));

        let (width, height, _) = difficulty_map.get(&difficuty).unwrap();
        for y in 0..*height {
            for x in 0..*width {
                tiles.push(Tile::new(Self::calculate_index(x, y, *width)));
            }
        }

        Self {
            state: GameState::Ready,
            rng,
            tiles,
            difficulty_map,
            first_click: true,
            difficuty,
        }
    }

    #[wasm_bindgen(js_name = getState)]
    pub fn get_state(&self) -> GameState {
        self.state
    }

    #[wasm_bindgen(js_name = getWidth)]
    pub fn get_width(&self) -> usize {
        self.difficulty_map.get(&self.difficuty).unwrap().0
    }

    #[wasm_bindgen(js_name = getHeight)]
    pub fn get_height(&self) -> usize {
        self.difficulty_map.get(&self.difficuty).unwrap().1
    }

    #[wasm_bindgen(js_name = getMines)]
    pub fn get_mines(&self) -> usize {
        self.difficulty_map.get(&self.difficuty).unwrap().2
    }

    fn genrate_mines(&mut self, init_index: usize, num_mine: usize) {
        let mut random_palce = || -> bool {
            let mut index = init_index;
            while index == init_index {
                index = self.rng.gen_range(0..self.get_width() * self.get_height());
            }
            if let Some(tile) = self.tiles.get_mut(index) {
                match tile.mine {
                    Some(mine) if mine => false,
                    _ => {
                        tile.mine.replace(true);
                        true
                    }
                }
            } else {
                false
            }
        };

        for _ in 0..num_mine {
            let mut placed = false;
            while !placed {
                placed = random_palce();
            }
        }
        self.update_numbers();
        self.state = GameState::Playing;
        debug("Game start");
    }

    #[wasm_bindgen(js_name = getTiles)]
    pub fn get_tiles(&self) -> Vec<Tile> {
        self.tiles.clone()
    }

    #[wasm_bindgen(js_name = onClick)]
    pub fn on_click(&mut self, index: usize, left: bool) {
        if self.state != GameState::Ready && self.state != GameState::Playing {
            debug("not playing");
            return;
        }
        debug(&format!("click index {}", index));
        let mine = self.tiles.get_mut(index).unwrap();
        if left {
            mine.revealed = true;
            if self.first_click {
                self.genrate_mines(index, self.get_mines());
                self.first_click = false;
            }
            self.expend_zero_tile(index);
        } else {
            mine.flagged = Some(true);
        }
        self.update_state();
    }

    fn update_state(&mut self) {
        let mut cnt = 0;
        for tile in &self.tiles {
            // if any mine is revealed, then lost
            if tile.has_mine() && tile.is_revealed() {
                self.state = GameState::Lost;
                debug("lost");
                return;
            }
            // count tiels that are not mine and not revealed
            if !tile.has_mine() && !tile.is_revealed() {
                cnt += 1;
            }
        }
        if cnt == 0 {
            self.state = GameState::Won;
            debug("won");
        }
    }

    fn update_numbers(&mut self) {
        for x in 0..self.get_width() {
            for y in 0..self.get_height() {
                let tile = self.get(x, y).unwrap();
                if tile.has_mine() {
                    continue;
                }
                let adjacent_mines = self
                    .get_siblings(tile.index)
                    .iter()
                    .map(|t| if t.has_mine() { 1 } else { 0 })
                    .fold(0, |acc, a| acc + a);
                self.get_mut(x, y).unwrap().adjacent_mines = adjacent_mines;
            }
        }
    }

    fn expend_zero_tile(&mut self, index: usize) {
        let tile = self.tiles.get_mut(index);
        if let Some(tile) = tile {
            if tile.adjacent_mines != 0 {
                return;
            }
            let sliblings: Vec<usize> = self.get_siblings(index).iter().map(|s| s.index).collect();
            for s in sliblings {
                let tile = self.tiles.get_mut(s).unwrap();
                if !tile.revealed {
                    if !tile.is_flagged() {
                        tile.revealed = true;
                    }
                    let index = tile.index;
                    self.expend_zero_tile(index);
                }
            }
        }
    }

    fn get_siblings(&self, index: usize) -> Vec<&Tile> {
        DIERECTIONS
            .iter()
            .map(|(dx, dy)| {
                let (x, y) = Self::calculate_loc(index, self.get_width());
                let x = x as i32 + dx;
                let y = y as i32 + dy;
                if x < 0 || x >= self.get_width() as i32 || y < 0 || y >= self.get_height() as i32 {
                    None
                } else {
                    self.get(x as usize, y as usize)
                }
            })
            .filter(|s| s.is_some())
            .map(|s| s.unwrap())
            .collect()
    }

    fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut Tile> {
        let index = Self::calculate_index(x, y, self.get_width());
        self.tiles.get_mut(index)
    }

    fn get(&self, x: usize, y: usize) -> Option<&Tile> {
        self.tiles
            .get(Self::calculate_index(x, y, self.get_width()))
    }

    fn calculate_index(x: usize, y: usize, width: usize) -> usize {
        y * width + x
    }

    fn calculate_loc(index: usize, width: usize) -> (usize, usize) {
        let x = index % width;
        let y = index / width;
        (x, y)
    }
}
