use super::*;
use serde::Deserialize;
use std::ops::Index;

/// The multi-reponse payload for `multi_search`.
#[derive(Debug, Clone, Deserialize)]
pub struct MultiResponse<T: Clone + std::fmt::Debug> {
    responses: Vec<SearchResults<T>>,
}

impl<T: Clone + std::fmt::Debug> Index<usize> for MultiResponse<T> {
    type Output = SearchResults<T>;

    fn index(&self, index: usize) -> &Self::Output {
        self.responses.index(index)
    }
}
