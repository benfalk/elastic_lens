//! All of the logic for parsing and working with a response is in this module.

mod multi_results;
mod search_results;
pub(crate) mod single_document;

pub use multi_results::*;
pub use search_results::*;
