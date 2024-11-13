use rs_sudoku_lib::error::SudokuError::*;
use rs_sudoku_lib::sudoku::{BaseSudoku, Board};

#[test]
fn test_board_new_1() {
    let board = Board::new(3, 10).unwrap();

    assert_eq!(board.board, vec![vec![0; 9]; 9]);
    assert_eq!(board.n, 3_u8);
    assert_eq!(board.e, 10_u8);
}

#[test]
fn test_board_new_2() {
    let board = Board::new(6, 30);

    assert_eq!(board.unwrap_err(), SizeExceedLimitError(36));
}

#[test]
fn test_board_rules_1() {
    let mut board = Board::new(3, 10).unwrap();
    board.board[0][0] = 1;

    assert_eq!(board.rules(0, 3, 1), false);
}

#[test]
fn test_board_rules_2() {
    let mut board = Board::new(3, 10).unwrap();
    board.board[0][0] = 1;

    assert_eq!(board.rules(3, 0, 1), false);
}

#[test]
fn test_board_rules_3() {
    let mut board = Board::new(3, 10).unwrap();
    board.board[0][0] = 1;

    assert_eq!(board.rules(1, 1, 1), false);
}

#[test]
fn test_board_rules_4() {
    let mut board = Board::new(3, 10).unwrap();
    board.board[0][0] = 1;

    assert_eq!(board.rules(1, 1, 2), true);
}

#[test]
fn test_generate_board() {
    let mut board = Board::new(3, 10).unwrap();
    board.generate_board(0);

    let expected = vec![
        vec![5, 1, 4, 6, 7, 3, 2, 8, 9],
        vec![9, 3, 2, 1, 4, 8, 6, 5, 7],
        vec![6, 7, 8, 2, 9, 5, 4, 3, 1],
        vec![1, 9, 5, 4, 2, 6, 3, 7, 8],
        vec![7, 8, 6, 3, 1, 9, 5, 2, 4],
        vec![2, 4, 3, 5, 8, 7, 9, 1, 6],
        vec![4, 6, 7, 8, 3, 2, 1, 9, 5],
        vec![3, 5, 9, 7, 6, 1, 8, 4, 2],
        vec![8, 2, 1, 9, 5, 4, 7, 6, 3],
    ];

    assert_eq!(board.board, expected);
}
