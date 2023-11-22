use rand::Rng;
use std::{thread, time};

fn main() {
    let mut y = Board::new(50, 50, '⬛', '⬜');
    y.run(100000);
}

struct Board {
    width: usize,
    height: usize,
    alive: char,
    dead: char,
    board: Vec<Vec<char>>,
    gen: u32,
}

impl Board {
    fn new(width: usize, height: usize, alive: char, dead: char) -> Self {
        // creates 2d vec with randomized chars
        let mut board: Vec<Vec<char>> = vec![vec![' '; height]; width];
        let mut rng = rand::thread_rng();
        for row in board.iter_mut() {
            for cell in row.iter_mut() {
                if rng.gen::<bool>() {
                    *cell = dead;
                } else {
                    *cell = alive;
                }
            }
        }
        Board {
            width,
            height,
            alive,
            dead,
            board,
            gen: 0,
        }
    }

    fn run(&mut self, generations: u32) {
        // run for x generations, replacing and printing out board
        for _ in 0..generations + 1 {
            self.check_cells();
            self.print_board();
            let ten_millis = time::Duration::from_millis(50);
            thread::sleep(ten_millis);
            clearscreen::clear().expect("failed to clear screen");
        }
    }

    fn check_cells(&mut self) {
        // all neighbors coordinates
        const NEIGHBOR_COORDINATES: [(i32, i32); 8] = [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];
        // clone previous board
        let mut new_board = self.board.clone();

        for i in 0..self.width {
            for j in 0..self.height {
                let mut live_neighbor_count = 0;

                // loops through neighbors
                for &(dx, dy) in &NEIGHBOR_COORDINATES {
                    let x = i as i32 + dx;
                    let y = j as i32 + dy;

                    // check if neighbor (x, y) is within bounds
                    if x >= 0 && x < self.width as i32 && y >= 0 && y < self.height as i32 {
                        if self.board[x as usize][y as usize] == self.alive {
                            live_neighbor_count += 1;
                        }
                    }
                }

                // game of life rules
                if self.board[i][j] == self.alive {
                    // cell with less than 2 or more than 3 neighbors dies
                    if live_neighbor_count < 2 || live_neighbor_count > 3 {
                        new_board[i][j] = self.dead;
                    }
                } else {
                    // dead cell with 3 neighbors is reborn
                    if live_neighbor_count == 3 {
                        new_board[i][j] = self.alive;
                    }
                }
            }
        }
        // set generation
        self.gen += 1;
        // replace board with new one
        self.board = new_board;
    }

    fn print_board(&self) {
        println!("Generation: {}", self.gen);
        for i in 0..self.board.len() {
            for x in 0..self.board[i].len() {
                print!("{}", self.board[i][x]);
            }
            print!("\n");
        }
    }
}
