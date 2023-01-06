//! All of the logic for parsing and working with a response is in this module.

pub(crate) mod single_document;
mod search_results;

pub use search_results::*;
