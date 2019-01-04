extern crate ndarray;
use ndarray::{ArrayD, IxDyn};
use std::io::{stdin,stdout, Write, BufWriter};

struct Hand {
    y: usize,
    x: usize,
}

#[derive(Debug)]
struct State {
    winner: i8,
    message: String,
}

struct Board {
    height: usize,
    width: usize,
    cells: ArrayD<i8>,
    turn: i8,
    is_finished: bool,
    state: Option<State>,
}
fn get_other_player(player_index: i8) {}

impl Board {
    pub fn new(height: usize, width: usize) -> Board {
        Board {
            height: height,
            width: width,
            cells: ArrayD::<i8>::zeros(IxDyn(&[height, width])),
            turn: 1,
            is_finished: false,
            state: None,
        }
    }
    pub fn update(self: &mut Board, hand: &Hand) {
        if hand.y < 0 || hand.y >= self.height || hand.x < 0 || hand.x >= self.width {
            let winner = 1 - self.turn & 1;
            self.state = Some(State {
                winner: winner,
                message: format!("player {} won: invalid hand", winner),
            });
            self.is_finished = true;
        } else if self.cells[[hand.y, hand.x]] != 0 {
            let winner = 1 - self.turn & 1;
            self.state = Some(State {
                winner: winner,
                message: format!("player {} won: invalid hand", winner),
            });
            self.is_finished = true;
        } else {
            self.cells[[hand.y, hand.x]] = 2 * (self.turn & 1) - 1;
            self.turn += 1;
        }
    }
    pub fn debug(self: &Board) {
        let out = stdout();
        let mut out = BufWriter::new(out.lock());
        // TODO(matsumoto): 19路用のハードコーディングを避ける
        writeln!(out, "  |  1  2  3  4  5  6  7  8  9 10 11 12 13 14 15 16 17 18 19").unwrap();
        writeln!(out, "--+---------------------------------------------------------").unwrap();
        for i in 0..self.height {
            write!(out,  "{:>2}| ", i + 1).unwrap();
            for j in 0..self.width {
                let c = if self.cells[[i, j]] == 1 {
                    "o"
                }else if self.cells[[i, j]] == -1 {
                    "x"
                }else {"-"};
                write!(out, "{:>2} ", c).unwrap();
            }
            writeln!(out, "").unwrap();
        }
        writeln!(out, "").unwrap();
    }
}

fn get_hand(board: &Board) -> Hand {
    Hand { y: 2, x: 2 }
}

fn get_human_hand(board: &Board) -> Hand {
    board.debug();
    print!("input your hand. y x >> ");
    stdout().flush().unwrap();
    let mut buf = String::new();
    stdin()
        .read_line(&mut buf)
        .expect("Failed to read line");
    let mut yx_iter = buf.split_whitespace();
    let y: usize = yx_iter.next().unwrap().parse().unwrap();
    let x: usize = yx_iter.next().unwrap().parse().unwrap();
    Hand { y: y-1, x: x-1 }
}

fn main() {
    println!("Hello, world!");
    const H: usize = 19;
    const W: usize = 19;
    let mut board = Board::new(H, W);
    while !board.is_finished {
        let hand = if board.turn & 1 == 1 {
            get_human_hand(&board)
        } else {
            get_hand(&board)
        };
        board.update(&hand);
    }
    board.debug();
    println!("{:?}", board.state);
}
