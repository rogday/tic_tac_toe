use itertools::Itertools;
use std::io;

#[derive(Debug)]
enum MoveError {
    PlaceIsOccupied,
    IndexOutOfRange,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Player {
    X,
    O,
}

type Board = [Option<Player>; 9];

#[derive(Debug, Copy, Clone)]
struct TicTacToe {
    board: Board,
    turn: Player,
    ai: Player,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Move {
    score: i64,
    index: usize,
}

impl TicTacToe {
    fn new() -> Self {
        TicTacToe {
            board: [None; 9],
            turn: Player::X,
            ai: Player::O,
        }
    }

    fn reset(&mut self) {
        *self = TicTacToe::new();
    }

    fn player_move(&mut self, place: usize) -> Result<Option<Player>, MoveError> {
        if !self.board[place].is_none() {
            return Err(MoveError::PlaceIsOccupied);
        }

        self.board[place] = Some(self.turn);
        self.turn = TicTacToe::turn(self.turn);

        Ok(TicTacToe::check_win(&self.board))
    }

    fn ai_move(&mut self, debug: bool) -> Option<Player> {
        self.ai = self.turn;
        let m = self.get_best_move(self.board, true, 0);
        if debug {
            println!("{:?}", m);
        }
        self.player_move(m.index).ok().unwrap()
    }

    fn turn(turn: Player) -> Player {
        match turn {
            Player::X => Player::O,
            Player::O => Player::X,
        }
    }

    fn maxmin_turn(&self, maximizing: bool) -> Option<Player> {
        Some(if maximizing {
            self.ai
        } else {
            TicTacToe::turn(self.ai)
        })
    }

    fn check_win(board: &[Option<Player>]) -> Option<Player> {
        for i in 0..=3 {
            let k = board.len() - 3 - (2 * i) + 1;
            for j in 0..k {
                if !board[j].is_none() && (j..).step_by(i + 1).take(3).map(|x| board[x]).all_equal()
                {
                    return board[j].map(TicTacToe::turn);
                }
            }
        }
        None
    }

    fn get_best_move(&mut self, mut board: Board, maximizing: bool, depth: usize) -> Move {
        if let Some(player) = TicTacToe::check_win(&board) {
            return Move {
                score: (if player == self.ai { 1 } else { -1 }) * 100 + depth as i64,
                index: 0,
            };
        }

        let mut best_move = Move {
            score: if maximizing {
                std::i64::MIN
            } else {
                std::i64::MAX
            },
            index: 0,
        };

        for i in 0..board.len() {
            if !board[i].is_none() {
                continue;
            }

            board[i] = self.maxmin_turn(maximizing);
            let mut new_move = self.get_best_move(board.clone(), !maximizing, depth + 1);

            new_move.index = i;
            board[i] = None;

            if maximizing {
                best_move = std::cmp::max(best_move, new_move);
            } else {
                best_move = std::cmp::min(best_move, new_move);
            }
        }

        best_move
    }
}

fn main() {
    let mut game = TicTacToe::new();

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let n: usize = input.trim().parse().unwrap();

        let res = match n {
            0..=8 => game.player_move(n),
            9 => {
                game.reset();
                continue;
            }
            10 => Ok(game.ai_move(true)),
            11 => return,
            _ => Err(MoveError::IndexOutOfRange),
        };

        match res {
            Ok(game_result) => {
                println!("{:?}", game.board);
                if let Some(winner) = game_result {
                    println!("Player {:?} won!", winner);
                }
            }
            Err(error) => {
                println!("Error: {:?}", error);
            }
        }
    }
}
