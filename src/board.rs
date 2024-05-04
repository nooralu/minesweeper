use rand::{rngs::ThreadRng, Rng};

use crate::settings::DIERECTIONS;

pub struct Tile {
    x: usize,
    y: usize,
    revealed: bool,
    adjacent_mines: usize,
    mine: Option<bool>,
    flagged: Option<bool>,
}

impl Tile {
    pub fn new(x: usize, y: usize) -> Self {
        Self {
            x,
            y,
            revealed: false,
            adjacent_mines: 0,
            mine: None,
            flagged: None,
        }
    }

    pub fn has_mine(&self) -> bool {
        self.mine.is_some_and(|mine| mine)
    }

    pub fn get_adjacent_mines(&self) -> usize {
        self.adjacent_mines
    }

    pub fn is_revealed(&self) -> bool {
        self.revealed
    }
}

pub struct Board {
    width: usize,
    height: usize,
    tiles: Vec<Vec<Tile>>,
    rng: ThreadRng,
}

impl Board {
    pub fn new(width: usize, height: usize) -> Self {
        let rng = rand::thread_rng();
        let mut tiles = vec![];

        for y in 0..height {
            let mut row = vec![];
            for x in 0..width {
                row.push(Tile::new(x, y));
            }
            tiles.push(row);
        }
        Self {
            width,
            height,
            rng,
            tiles,
        }
    }

    pub fn genrate_mines(&mut self, num_mine: usize) {
        let mut random_palce = || -> bool {
            let x: usize = self.rng.gen_range(0..self.width);
            let y = self.rng.gen_range(0..self.height);
            let tile = self.get_mut(x, y);
            if let Some(tile) = tile {
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

    pub fn get_tiles(&self) -> &Vec<Vec<Tile>> {
        &self.tiles
    }

    pub fn on_click(&mut self, x: usize, y: usize) {
        let mine = self.get_mut(x, y).unwrap();
        mine.revealed = true;
    }

    fn update_numbers(&mut self) {
        for x in 0..self.width {
            for y in 0..self.height {
                let tile = self.get(x, y).unwrap();
                if tile.has_mine() {
                    continue;
                }
                let adjacent_mines = self
                    .get_siblings(&tile)
                    .iter()
                    .map(|t| if t.has_mine() { 1 } else { 0 })
                    .fold(0, |acc, a| acc + a);
                self.get_mut(x, y).unwrap().adjacent_mines = adjacent_mines;
            }
        }
    }

    fn get_siblings(&self, tile: &Tile) -> Vec<&Tile> {
        DIERECTIONS
            .iter()
            .map(|(dx, dy)| {
                let x = tile.x as i32 + dx;
                let y = tile.y as i32 + dy;
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
        match self.tiles.get_mut(y) {
            Some(row) => row.get_mut(x),
            _ => None,
        }
    }

    fn get(&self, x: usize, y: usize) -> Option<&Tile> {
        match self.tiles.get(y) {
            Some(row) => row.get(x),
            _ => None,
        }
    }
}
