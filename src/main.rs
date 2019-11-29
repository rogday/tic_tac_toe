use std::io;

mod game {
    #[derive(Debug)]
    pub enum MoveError {
        PlaceIsOccupied,
        IndexOutOfRange,
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum Player {
        X,
        O,
    }
    type Cell = Option<Player>;
    type Board = Vec<Cell>;

    #[derive(Debug)]
    pub struct TicTacToe {
        board: Board,
        turn: Player,
        ai: Player,
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
    struct Move {
        score: i64,
        index: usize,
    }

    fn turn(turn: Player) -> Player {
        match turn {
            Player::X => Player::O,
            Player::O => Player::X,
        }
    }

    fn check_win(board: &[Cell]) -> Option<Player> {
        use itertools::Itertools;

        let max_distance = (board.len() - 3) / 2;
        for d in 0..=max_distance {
            let last_occurence = board.len() - 3 - (2 * d);
            for j in 0..=last_occurence {
                if !board[j].is_none() && (j..).step_by(d + 1).take(3).map(|x| board[x]).all_equal()
                {
                    return board[j].map(turn);
                }
            }
        }
        None
    }

    impl TicTacToe {
        pub fn with_size(size: usize) -> Self {
            TicTacToe {
                board: std::vec::from_elem(None, size),
                turn: Player::X,
                ai: Player::O,
            }
        }

        pub fn board(&self) -> &Board {
            &self.board
        }

        pub fn reset(&mut self) {
            *self = TicTacToe::with_size(self.board.len());
        }

        pub fn player_move(&mut self, place: usize) -> Result<Option<Player>, MoveError> {
            if !self.board[place].is_none() {
                return Err(MoveError::PlaceIsOccupied);
            }

            self.board[place] = Some(self.turn);
            self.turn = turn(self.turn);

            Ok(check_win(&self.board))
        }

        pub fn ai_move(&mut self, debug: bool) -> Result<Option<Player>, MoveError> {
            self.ai = self.turn;
            let m = self.get_best_move(self.board.clone(), std::i64::MIN, std::i64::MAX, true, 0);
            if debug {
                println!("{:?}", m);
            }
            self.player_move(m.index)
        }

        fn maxmin_turn(&self, maximizing: bool) -> Option<Player> {
            Some(if maximizing { self.ai } else { turn(self.ai) })
        }

        fn get_best_move(
            &mut self,
            mut board: Board,
            mut alpha: i64,
            mut beta: i64,
            maximizing: bool,
            depth: usize,
        ) -> Move {
            println!(
                "{padding}{}: {}",
                depth,
                board
                    .iter()
                    .map(|x| match x {
                        None => '_',
                        Some(Player::O) => 'O',
                        Some(Player::X) => 'X',
                    })
                    .collect::<String>(),
                padding = std::iter::repeat('\t').take(depth).collect::<String>()
            );

            if let Some(winner) = check_win(&board) {
                println!("{:?}", winner);
                return Move {
                    score: (if winner == self.ai { 1 } else { -1 }) * 100 + depth as i64,
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
                let mut new_move =
                    self.get_best_move(board.clone(), alpha, beta, !maximizing, depth + 1);

                new_move.index = i;
                board[i] = None;

                if maximizing {
                    best_move = std::cmp::max(best_move, new_move);
                    alpha = std::cmp::max(alpha, new_move.score);
                } else {
                    best_move = std::cmp::min(best_move, new_move);
                    beta = std::cmp::min(beta, new_move.score);
                }

                if beta <= alpha {
                    break;
                }
            }

            println!("{}{:?}", depth, best_move);
            best_move
        }
    }
}

fn read<T: std::str::FromStr>() -> T
where
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().parse().unwrap()
}

fn main() {
    use game::*;

    let size = read();
    let mut game = TicTacToe::with_size(size);

    loop {
        let n = read();

        let res = match n {
            0..=8 => game.player_move(n),
            9 => {
                game.reset();
                continue;
            }
            10 => game.ai_move(true),
            11 => return,
            _ => Err(MoveError::IndexOutOfRange),
        };

        match res {
            Ok(game_result) => {
                println!(
                    "{}",
                    game.board()
                        .iter()
                        .map(|x| match x {
                            None => '_',
                            Some(Player::O) => 'O',
                            Some(Player::X) => 'X',
                        })
                        .collect::<String>()
                );
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
