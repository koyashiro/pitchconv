use std::num::ParseIntError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParsePitchClassError;

impl From<ParseIntError> for ParsePitchError {
    fn from(_: ParseIntError) -> Self {
        ParsePitchError
    }
}

impl From<ParsePitchClassError> for ParsePitchError {
    fn from(_: ParsePitchClassError) -> Self {
        ParsePitchError
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParsePitchError;
