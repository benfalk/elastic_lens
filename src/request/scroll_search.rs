use std::borrow::Cow;

use super::search::{SearchBody, SearchTrait};

/// Scroll Search
///
/// Responsible for starting of cursor based
/// search in which the server issues a scroll
/// id with which you can continue to fetch
/// results from until the results from the original
/// search are exhausted.
///
#[derive(Debug)]
pub struct ScrollSearch<'a> {
    pub(crate) search_body: SearchBody<'a>,
    pub(crate) duration: Cow<'static, str>,
}

impl<'a> ScrollSearch<'a> {
    /// Creates a search with a default 1 minute keepalive
    pub fn new<T: SearchTrait>(search: &'a T) -> Self {
        Self::new_with_duration(search, "1m")
    }

    /// Creates a search with a given keepalive time limit.
    ///
    /// The limit is the amount of time between fetches in which
    /// Elasticsearch should keep results queued for more fetches.
    /// The format is in the following format:
    ///
    /// - "1m" == one minute
    /// - "25s" == twenty five seconds
    /// - "6h" == six hours
    ///
    pub fn new_with_duration<T, D>(search: &'a T, duration: D) -> Self
    where
        T: SearchTrait,
        D: Into<Cow<'static, str>>,
    {
        Self {
            search_body: search.search_body(),
            duration: duration.into(),
        }
    }
}

/// Scroll Cursor
///
/// Produced by [crate::client::Client::scroll_search]
/// and used by [crate::client::Client::scroll] to
/// fetch the next page of results.
///
#[derive(Debug)]
pub struct ScrollCursor {
    pub(crate) scroll_id: String,
    pub(crate) scroll: Cow<'static, str>,
}
