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

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum GameResult {
        Win(Player),
        Tie,
        NotEnded,
    }

    type Cell = Option<Player>;
    pub type Board = Vec<Cell>;

    #[derive(Debug)]
    pub struct TicTacToe {
        board: Board,
        turn: Player,
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

    fn check_win(board: &[Cell]) -> GameResult {
        use itertools::Itertools;

        let max_distance = (board.len() - 3) / 2;
        for d in 0..=max_distance {
            let last_occurence = board.len() - 3 - (2 * d);
            for j in 0..=last_occurence {
                if !board[j].is_none() && (j..).step_by(d + 1).take(3).map(|x| board[x]).all_equal()
                {
                    return GameResult::Win(board[j].map(turn).unwrap());
                }
            }
        }

        //draw check which should be useless after certain size of the board
        if board.iter().filter(|x| x.is_none()).count() == 0 {
            return GameResult::Tie;
        }

        GameResult::NotEnded
    }

    impl TicTacToe {
        pub fn with_size(size: usize) -> Self {
            TicTacToe {
                board: vec![None; size],
                turn: Player::X,
            }
        }

        pub fn board(&self) -> &Board {
            &self.board
        }

        pub fn reset(&mut self) {
            *self = TicTacToe::with_size(self.board.len());
        }

        pub fn player_move(&mut self, place: usize) -> Result<GameResult, MoveError> {
            if !self.board[place].is_none() {
                return Err(MoveError::PlaceIsOccupied);
            }

            self.board[place] = Some(self.turn);
            self.turn = turn(self.turn);

            Ok(check_win(&self.board))
        }

        pub fn ai_move(&mut self, debug: bool) -> Result<GameResult, MoveError> {
            let m = self.get_best_move(self.board.clone(), std::i64::MIN, std::i64::MAX, true, 0);
            if debug {
                println!("{:?}", m);
            }
            self.player_move(m.index)
        }

        fn maxmin_turn(&self, maximizing: bool) -> Option<Player> {
            Some(if maximizing {
                self.turn
            } else {
                turn(self.turn)
            })
        }

        fn get_best_move(
            &self,
            board: Board,
            mut alpha: i64,
            mut beta: i64,
            maximizing: bool,
            depth: usize,
        ) -> Move {
            match check_win(&board) {
                GameResult::Win(winner) => {
                    return Move {
                        score: (if winner == self.turn { 1 } else { -1 }) * 100
                            + (if maximizing { -1 } else { 1 }) * depth as i64,
                        index: 0,
                    }
                }
                GameResult::Tie => return Move { score: 0, index: 0 },
                GameResult::NotEnded => (),
            }

            let mut best_move = Move {
                score: if maximizing {
                    std::i64::MIN
                } else {
                    std::i64::MAX
                },
                index: 0,
            };

            let mark = self.maxmin_turn(maximizing);
            for i in 0..board.len() {
                if !board[i].is_none() {
                    continue;
                }

                let mut new_board = board.clone();
                new_board[i] = mark;

                let mut new_move =
                    self.get_best_move(new_board, alpha, beta, !maximizing, depth + 1);

                new_move.index = i;

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

fn board_to_string(board: &game::Board) -> String {
    board
        .iter()
        .map(|x| match x {
            None => '_',
            Some(game::Player::O) => 'O',
            Some(game::Player::X) => 'X',
        })
        .collect()
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
                println!("{}", board_to_string(game.board()));

                match game_result {
                    GameResult::Win(winner) => println!("Player {:?} won!", winner),
                    GameResult::Tie => println!("Tie!"),
                    GameResult::NotEnded => (),
                }
            }
            Err(error) => {
                println!("Error: {:?}", error);
            }
        }
    }
}
