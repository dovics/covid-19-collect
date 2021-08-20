use std::fmt;
use fmt::{Display, Formatter};

#[derive(Debug)]
pub struct PostgresError {
    pub kind: PostgresErrorKind,
    pub message: String
}

#[derive(Debug)]
pub enum PostgresErrorKind {
    CreateTableError = 1,
    InsertData = 2,
}

impl Display for PostgresError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}: {:?}", self.kind, self.message)
    }
}

impl std::error::Error for PostgresError {}