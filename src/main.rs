use board::Board;
use settings::{HEIGHT, NUM_MINE, WIDTH};
mod board;
mod settings;

fn main() {
    let mut board = Board::new(WIDTH, HEIGHT);
    board.genrate_mines(NUM_MINE);
    board.display();

    loop {
        let mut cmd = String::new();
        std::io::stdin()
            .read_line(&mut cmd)
            .expect("Failed to read line");

        let locs: Vec<usize> = cmd
            .trim()
            .split_ascii_whitespace()
            .map(|item| item.parse::<usize>().unwrap())
            .collect();

        board.on_click(*locs.get(0).unwrap(), *locs.get(1).unwrap());
        board.display();
    }
}

impl Board {
    pub fn display(&self) {
        for row in self.get_tiles() {
            for tile in row {
                if !tile.is_revealed() {
                    print!("*");
                    continue;
                }
                if tile.has_mine() {
                    print!("b");
                } else {
                    print!("{}", tile.get_adjacent_mines());
                }
            }
            println!();
        }
    }
}
