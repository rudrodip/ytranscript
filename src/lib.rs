/// The `errors` module defines the error types for the `ytranscript` crate.
pub mod errors;

/// The `fetch` module provides the functionality to fetch YouTube transcripts.
pub mod fetch;

/// The `regex` module defines the regular expression patterns used in the `ytranscript` crate.
pub mod regex;

/// The `types` module defines the data structures used in the `ytranscript` crate.
pub mod types;

// Re-export the modules for easier access
pub use crate::errors::*;
pub use crate::fetch::*;
pub use crate::regex::*;
pub use crate::types::*;
