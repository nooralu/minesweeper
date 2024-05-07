use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
#[derive(Clone, Copy)]
pub struct Tile {
    pub(crate) index: usize,
    pub(crate) revealed: bool,
    pub(crate) adjacent_mines: usize,
    pub(crate) mine: Option<bool>,
    pub(crate) flagged: Option<bool>,
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
