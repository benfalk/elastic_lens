use super::*;
use serde::Deserialize;
use std::fmt::Debug;
use std::ops::Index;

/// The multi-reponse payload for `multi_search`.
#[derive(Deserialize)]
pub struct MultiResponse<T> {
    responses: Vec<SearchResults<T>>,
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
