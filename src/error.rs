use std::io;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum AocSolveErrkind {
    #[error("{0}")]
    IoErr(#[from] io::Error),
    #[error("parse failed")]
    ParseFailed,
    #[error("error occurs")]
    ErrWithNoInfo,
}

pub type Result<T> = std::result::Result<T, AocSolveErrkind>;
