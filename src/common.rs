//! Types needed in multiple modules

/// Enumeration of binary symbol values
#[derive(Clone, Eq, PartialEq, Debug, Copy)]
pub enum Bit {
    /// Binary symbol `0`
    Zero = 0,

    /// Binary symbol `1`
    One = 1,
}

impl From<bool> for Bit {
    fn from(value: bool) -> Self {
        if value {
            Self::One
        } else {
            Self::Zero
        }
    }
}

impl From<Bit> for bool {
    fn from(value: Bit) -> Self {
        match value {
            Bit::Zero => false,
            Bit::One => true,
        }
    }
}

/// Custom error type
#[derive(thiserror::Error, Debug)]
pub enum Error {
    /// Invalid input error
    #[error("{0}")]
    InvalidInput(String),

    /// File read/write error
    #[cfg(feature = "cli")]
    #[error("{0}")]
    FileReadWriteError(#[from] std::io::Error),

    /// Serde read/write error
    #[cfg(feature = "cli")]
    #[error("{0}")]
    SerdeReadWriteError(#[from] serde_json::Error),

    /// Unknown error
    #[error("Unknown error")]
    Unknown,
}
