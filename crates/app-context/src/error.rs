use std::{error::Error, fmt};

/// A registration or extraction error. Contains type names, never stored values.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum ContextError {
    /// The requested type has not been registered.
    Missing {
        /// Rust diagnostic name of the requested type.
        type_name: &'static str,
    },
    /// Insertion would overwrite an existing registration.
    Duplicate {
        /// Rust diagnostic name of the already registered type.
        type_name: &'static str,
    },
}

impl fmt::Display for ContextError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Missing { type_name } => write!(
                f,
                "no state registered for `{type_name}`; insert it before building the context"
            ),
            Self::Duplicate { type_name } => write!(
                f,
                "state already registered for `{type_name}`; use a newtype or explicitly replace it"
            ),
        }
    }
}

impl Error for ContextError {}
