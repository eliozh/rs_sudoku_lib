use std::fmt;

#[derive(Debug, PartialEq)]
pub enum SudokuError {
    SizeExceedLimitError(u8),
}

impl fmt::Display for SudokuError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SudokuError::SizeExceedLimitError(v) => {
                write!(
                    f,
                    "Size exceed the limit, expected less than 25, got {}.",
                    v
                )
            }
        }
    }
}
