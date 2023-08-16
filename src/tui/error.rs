use std::error::Error;
use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Clone)]
pub enum TuiError {
    ChannelClosed(String),
}

impl Display for TuiError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Self::ChannelClosed(s) => write!(f, "ChannelClosed: {}", s)
        }
    }
}

impl Error for TuiError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}
