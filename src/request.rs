//! All of the logic surrounding creating a request is held in this module.

pub mod search;

mod multi_search;
pub use multi_search::*;

mod scroll_search;
pub use scroll_search::*;
