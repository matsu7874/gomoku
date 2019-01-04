extern crate ndarray;
extern crate rand;

use ndarray::{Array2, Ix2};
use rand::prelude::*;
use std::io::{stdin, stdout, BufWriter, Write};

const H: usize = 19;
const W: usize = 19;

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
    cells: Array2<i8>,
    turn: usize,
    is_finished: bool,
    state: Option<State>,
}

impl Board {
    pub fn new() -> Board {
        Board {
            cells: Array2::<i8>::zeros(Ix2(H, W)),
            turn: 1,
            is_finished: false,
            state: None,
        }
    }

    pub fn update(self: &mut Board, hand: &Hand) {
        let player_index = 2 * (self.turn & 1) as i8 - 1;
        // 石を置く
        if !self.is_valid(hand) {
            let winner_index = -player_index;
            self.state = Some(State {
                winner: winner_index,
                message: format!("player {} won", &Board::pleyer_index_to_char(winner_index)),
            });
            self.is_finished = true;
        } else {
            println!(
                "{} {} {}",
                &Board::pleyer_index_to_char(player_index),
                hand.y + 1,
                hand.x + 1
            );
            self.cells[[hand.y, hand.x]] = player_index;
        }
        // ゲームの終了判定
        if self.is_player_won(hand) {
            self.is_finished = true;
            self.state = Some(State {
                winner: player_index,
                message: format!("player {} won", &Board::pleyer_index_to_char(player_index)),
            });
        } else if self.turn == H * W {
            self.is_finished = true;
            self.state = Some(State {
                winner: 0,
                message: format!("draw"),
            });
        }
        self.turn += 1;
    }

    pub fn is_valid(self: &Board, hand: &Hand) -> bool {
        if hand.y < 0 || hand.y >= H || hand.x < 0 || hand.x >= W {
            return false;
        } else if self.cells[[hand.y, hand.x]] != 0 {
            return false;
        } else {
            return true;
        }
    }

    pub fn is_player_won(self: &Board, hand: &Hand) -> bool {
        let player_index = 2 * (self.turn & 1) as i8 - 1;

        let mut cnt_connect = Array2::<usize>::zeros(Ix2(3, 3));
        for dy in -1..2 {
            for dx in -1..2 {
                if dy == 0 && dx == 0 {
                    continue;
                }
                // 今打った石の周り４マスまでを辿る。
                for i in 1..5 {
                    if hand.y as i8 + dy * i < 0
                        || H as i8 <= hand.y as i8 + dy * i
                        || hand.x as i8 + dx * i < 0
                        || W as i8 <= hand.x as i8 + dx * i
                    {
                        break;
                    }
                    if self.cells[[
                        (hand.y as i8 + dy * i) as usize,
                        (hand.x as i8 + dx * i) as usize,
                    ]] == player_index
                    {
                        cnt_connect[[(dy + 1) as usize, (dx + 1) as usize]] += 1;
                    } else {
                        break;
                    }
                }
            }
        }
        for ((sy, sx), (ty, tx)) in vec![
            ((0, 0), (2, 2)),
            ((0, 1), (2, 1)),
            ((0, 2), (2, 0)),
            ((1, 0), (1, 2)),
        ] {
            if cnt_connect[[sy, sx]] + 1 + cnt_connect[[ty, tx]] >= 5 {
                return true;
            }
        }
        false
    }

    pub fn display_board(self: &Board) {
        let out = stdout();
        let mut out = BufWriter::new(out.lock());
        // TODO(matsumoto): 19路用のハードコーディングを避ける
        writeln!(
            out,
            "  |  1  2  3  4  5  6  7  8  9 10 11 12 13 14 15 16 17 18 19"
        )
        .unwrap();
        writeln!(
            out,
            "--+---------------------------------------------------------"
        )
        .unwrap();
        for i in 0..H {
            write!(out, "{:>2}| ", i + 1).unwrap();
            for j in 0..W {
                let c = &Board::pleyer_index_to_char(self.cells[[i, j]]);
                write!(out, "{:>2} ", c).unwrap();
            }
            writeln!(out, "").unwrap();
        }
        writeln!(out, "").unwrap();
    }
    fn pleyer_index_to_char(player_index: i8) -> &'static str {
        if player_index == -1 {
            "x"
        } else if player_index == 1 {
            "o"
        } else {
            "-"
        }
    }
}

fn get_hand(board: &Board) -> Hand {
    let y = random::<usize>() % H;
    let x = random::<usize>() % W;
    let mut hand = Hand { y: y, x: x };
    while !board.is_valid(&hand) {
        hand.y = random::<usize>() % H;
        hand.x = random::<usize>() % W;
    }
    hand
}

#[allow(dead_code)]
fn get_human_hand(board: &Board) -> Hand {
    board.display_board();
    print!("input your hand. y x >> ");
    stdout().flush().unwrap();
    let mut buf = String::new();
    stdin().read_line(&mut buf).expect("Failed to read line");
    let mut yx_iter = buf.split_whitespace();
    let y: usize = yx_iter.next().unwrap().parse().unwrap();
    let x: usize = yx_iter.next().unwrap().parse().unwrap();
    Hand { y: y - 1, x: x - 1 }
}

fn main() {
    let mut board = Board::new();
    while !board.is_finished {
        let hand = if board.turn & 1 == 1 {
            // get_human_hand(&board)
            get_hand(&board)
        } else {
            get_hand(&board)
        };
        board.update(&hand);
    }

    if let Some(ref state) = board.state {
        println!("{:?}", state.message);
    }
    board.display_board();
}
