use rand::{rngs::ThreadRng, Rng};
use wasm_bindgen::prelude::*;

use crate::{debug, settings::DIERECTIONS};

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
    width: usize,
    height: usize,
    tiles: Vec<Tile>,
    rng: ThreadRng,
    first_click: bool,
}

#[wasm_bindgen]
impl Board {
    #[wasm_bindgen(constructor)]
    pub fn new(width: usize, height: usize) -> Self {
        let rng = rand::thread_rng();
        let mut tiles = vec![];

        for y in 0..height {
            for x in 0..width {
                tiles.push(Tile::new(Self::calculate_index(x, y, width)));
            }
        }

        Self {
            width,
            height,
            rng,
            tiles,
            first_click: true,
        }
    }

    fn genrate_mines(&mut self, init_index: usize, num_mine: usize) {
        let mut random_palce = || -> bool {
            let mut index = init_index;
            while index == init_index {
                index = self.rng.gen_range(0..self.width * self.height);
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
    }

    #[wasm_bindgen(js_name = getTiles)]
    pub fn get_tiles(&self) -> Vec<Tile> {
        self.tiles.clone()
    }

    #[wasm_bindgen(js_name = onClick)]
    pub fn on_click(&mut self, index: usize, left: bool) {
        debug(&format!("click index {}", index));
        let mine = self.tiles.get_mut(index).unwrap();
        if left {
            mine.revealed = true;
            if self.first_click {
                // TODO: handle magic number 3
                self.genrate_mines(index, 3);
                self.first_click = false;
            }
            self.expend_zero_tile(index);
        } else {
            mine.flagged = Some(true);
        }
    }

    fn update_numbers(&mut self) {
        for x in 0..self.width {
            for y in 0..self.height {
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
                let (x, y) = Self::calculate_loc(index, self.width);
                let x = x as i32 + dx;
                let y = y as i32 + dy;
                if x < 0 || x >= self.width as i32 || y < 0 || y >= self.height as i32 {
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
        let index = Self::calculate_index(x, y, self.width);
        self.tiles.get_mut(index)
    }

    fn get(&self, x: usize, y: usize) -> Option<&Tile> {
        self.tiles.get(Self::calculate_index(x, y, self.width))
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
