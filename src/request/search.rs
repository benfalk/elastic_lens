//!
//! The real inspiration for this library is a search building DSL that
//! is ergonomic and well documented.  Everything in this module aims
//! to accomplish that goal.
//!

mod aggregation;
mod body;
mod criterion;
mod field;
mod geo_values;
mod numeric_value;
mod scalar_value;
mod search_trait;
mod target;

pub use aggregation::*;
pub use body::*;
pub use criterion::*;
pub use field::*;
pub use geo_values::*;
pub use numeric_value::*;
pub use scalar_value::*;
pub use search_trait::*;
pub use target::*;

/// Used to construct a search request for Elasticsearch
#[derive(Debug, Clone, Default)]
pub struct Search {
    positive_criteria: Vec<Criterion>,
    negative_criteria: Vec<Criterion>,
    limit: Option<usize>,
    offset: Option<usize>,
}

impl Search {
    /// maximum amount of documents to return
    pub fn set_limit(&mut self, limit: usize) {
        self.limit = Some(limit);
    }

    /// offset from where to start returning documents
    pub fn set_offset(&mut self, offset: usize) {
        self.offset = Some(offset);
    }
}

impl SearchTrait for Search {
    fn limit(&self) -> Option<usize> {
        self.limit
    }

    fn offset(&self) -> Option<usize> {
        self.offset
    }

    fn positive_criteria(&self) -> Option<&Vec<Criterion>> {
        if self.positive_criteria.is_empty() {
            None
        } else {
            Some(&self.positive_criteria)
        }
    }

    fn negative_criteria(&self) -> Option<&Vec<Criterion>> {
        if self.negative_criteria.is_empty() {
            None
        } else {
            Some(&self.negative_criteria)
        }
    }
}

impl CriteriaBuilder for Search {
    type Bucket = PositiveBucket;

    fn positive_criteria_mut(&mut self) -> &mut Vec<Criterion> {
        &mut self.positive_criteria
    }

    fn negative_criteria_mut(&mut self) -> &mut Vec<Criterion> {
        &mut self.negative_criteria
    }
}
