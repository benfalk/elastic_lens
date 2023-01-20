use super::*;
use serde::Deserialize;
use std::fmt::Debug;
use std::ops::Index;

/// The multi-reponse payload for `multi_search`.
#[derive(Deserialize)]
pub struct MultiResponse<T> {
    responses: Vec<SearchResults<T>>,
}

impl<T> MultiResponse<T> {
    /// Retruns the actual data `MultiResponse` was working
    /// with; which is a vector of the search results.  In
    /// some cases when you want to perform more ownership
    /// based access on the data this is probably the best
    /// way to get at it.
    pub fn to_inner(self) -> Vec<SearchResults<T>> {
        self.responses
    }
}

impl<T> Index<usize> for MultiResponse<T> {
    type Output = SearchResults<T>;

    fn index(&self, index: usize) -> &Self::Output {
        self.responses.index(index)
    }
}

impl<T: Clone> Clone for MultiResponse<T> {
    fn clone(&self) -> Self {
        Self {
            responses: self.responses.clone(),
        }
    }
}

impl<T: Debug> Debug for MultiResponse<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MultiResponse")
            .field("responses", &self.responses)
            .finish()
    }
}
