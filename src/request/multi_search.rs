//! Logic around creating and sending multiple search requests at once

use super::search::{SearchBody, SearchTrait};

/// MultiSearch
///
/// This allows for the sending of multiple searches
/// inside of a single request to avoid IO round trips.
/// In it's current form it works with multiple different
/// `SearchTrait` at the same time as it's just holding
/// a reference to it's search body.
#[derive(Debug, Default)]
pub struct MultiSearch<'a> {
    pub(crate) bodies: Vec<SearchBody<'a>>,
}

/// This is an opaque type that is used
/// to retreive the contents of the search
/// for which it was returned when added to
/// `MultiSearch`
#[derive(Debug, Clone, Copy)]
pub struct LookupKey(usize);

impl<'a> MultiSearch<'a> {
    /// If you know how many requests you want to send at
    /// once this will help avoid excess allocations
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            bodies: Vec::with_capacity(capacity),
        }
    }

    /// Adds a search
    ///
    /// This queues to search to be included and returns
    /// a lookup key which will be needed to retreive the
    /// results back from the multi-response.
    pub fn add<T: SearchTrait>(&mut self, search: &'a T) -> LookupKey {
        self.bodies.push(search.search_body());
        LookupKey(self.bodies.len() - 1)
    }
}

impl<'a, B: Into<SearchBody<'a>>, I: IntoIterator<Item = B>> From<I> for MultiSearch<'a> {
    fn from(value: I) -> Self {
        let bodies = value.into_iter().map(Into::into).collect();

        Self { bodies }
    }
}
