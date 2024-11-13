use crate::error::SudokuError::{self, *};
use rand::prelude::*;
use rand::seq::SliceRandom;
use rand_chacha::ChaCha12Rng;

#[derive(Debug, PartialEq)]
pub struct Board {
    pub n: u8,
    pub e: u8,
    pub sqn: u8,
    pub board: Vec<Vec<u8>>,
}

/// Normal 9x9 sudoku
pub trait BaseSudoku {
    fn generate_board(&mut self, seed: u64);

    fn _generate_board(&mut self, cnt: usize, num: u8, row: usize, rng: &mut ChaCha12Rng) -> bool;

    fn rules(&self, row: usize, col: usize, num: u8) -> bool;
}

impl Board {
    //! Create a normal sudoku board.
    //! The number of columns and rows are the same,
    //! and the number of columns must be a square number.
    //!
    //! $n^2$ is the number of columns or rows,
    //! and `e` is the number of cells to remove.
    pub fn new(n: u8, e: u8) -> Result<Self, SudokuError> {
        if n > 5 {
            return Err(SizeExceedLimitError(n * n));
        }

        let sqn = n * n;

        Ok(Self {
            n,
            e,
            sqn,
            board: vec![vec![0; sqn as usize]; sqn as usize],
        })
    }
}

impl BaseSudoku for Board {
    //! Generate a sudoku board with given rules.
    fn generate_board(&mut self, seed: u64) {
        let mut rng = ChaCha12Rng::seed_from_u64(seed);

        self._generate_board(0, 1, 0, &mut rng);

        let sqn = self.sqn as usize;
        for row in 0..sqn {
            for col in 0..sqn {
                if self.board[row][col] == 0 {
                    self.board[row][col] = self.sqn;
                }
            }
        }

        // TODO: Remove some cells.
    }

    fn _generate_board(&mut self, cnt: usize, num: u8, row: usize, rng: &mut ChaCha12Rng) -> bool {
        let sqn = self.sqn as usize;

        if cnt == sqn.pow(2) - sqn {
            return true;
        }

        let mut col_indexes = (0..sqn).collect::<Vec<usize>>();
        col_indexes.shuffle(rng);

        for col in col_indexes {
            if self.board[row][col] != 0 {
                continue;
            }

            if self.rules(row, col, num) {
                self.board[row][col] = num;
                let result = if row == sqn - 1 {
                    self._generate_board(cnt + 1, num + 1, 0, rng)
                } else {
                    self._generate_board(cnt + 1, num, row + 1, rng)
                };

                if result {
                    return true;
                } else {
                    self.board[row][col] = 0;
                }
            }
        }

        false
    }

    fn rules(&self, row: usize, col: usize, num: u8) -> bool {
        let (n, sqn) = (self.n as usize, self.sqn as usize);

        // Validate current row
        for i in 0..sqn {
            if i == col {
                continue;
            }

            if self.board[row][i] == num {
                return false;
            }
        }

        // Validate current col
        for i in 0..sqn {
            if i == row {
                continue;
            }

            if self.board[i][col] == num {
                return false;
            }
        }

        // Validate current subblock
        let (sub_row, sub_col) = (row / n * n, col / n * n);
        for i in sub_row..(sub_row + 3) {
            for j in sub_col..(sub_col + 3) {
                if i == row && j == col {
                    continue;
                }

                if self.board[i][j] == num {
                    return false;
                }
            }
        }

        true
    }
}
