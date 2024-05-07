use std::usize;

use wasm_bindgen::prelude::*;

use crate::tile::Tile;
use crate::{debug, random, Difficulty, GameState, DIERECTIONS};

#[wasm_bindgen]
pub struct Board {
    state: GameState,
    tiles: Vec<Tile>,
    first_click: bool,
    difficuty: Difficulty,
}

#[wasm_bindgen]
impl Board {
    #[wasm_bindgen(constructor)]
    pub fn new(difficuty: Difficulty) -> Self {
        let mut tiles = vec![];

        let (width, height, _) = difficuty.get_config();
        for index in 0..width * height {
            tiles.push(Tile::new(index));
        }

        Self {
            state: GameState::Ready,
            tiles,
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
        self.difficuty.get_config().0
    }

    #[wasm_bindgen(js_name = getHeight)]
    pub fn get_height(&self) -> usize {
        self.difficuty.get_config().1
    }

    #[wasm_bindgen(js_name = getMines)]
    pub fn get_mines(&self) -> usize {
        self.difficuty.get_config().2
    }

    fn genrate_mines(&mut self, init_index: usize, num_mine: usize) {
        let mut random_palce = || -> bool {
            let mut index = init_index;
            while index == init_index {
                index = (random() * self.tiles.len() as f64) as usize;
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
        for index in 0..self.tiles.len() {
            let tile = self.tiles.get_mut(index).unwrap();
            if tile.has_mine() {
                continue;
            }

            let adjacent_mines = self
                .get_siblings(index)
                .iter()
                .map(|t| if t.has_mine() { 1 } else { 0 })
                .sum::<usize>();
            self.tiles.get_mut(index).unwrap().adjacent_mines = adjacent_mines;
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
        let width = self.get_width();
        DIERECTIONS
            .iter()
            .filter_map(|(dx, dy)| {
                let (x, y) = self.calculate_loc(index);
                let x = x as i32 + dx;
                let y = y as i32 + dy;
                if x < 0 || x >= self.get_width() as i32 || y < 0 || y >= self.get_height() as i32 {
                    None
                } else {
                    self.tiles.get(y as usize * width + x as usize)
                }
            })
            .collect()
    }

    fn calculate_loc(&self, index: usize) -> (usize, usize) {
        let width = self.get_width();
        let x = index % width;
        let y = index / width;
        (x, y)
    }
}
