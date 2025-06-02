//!
//! The real inspiration for this library is a search building DSL that
//! is ergonomic and well documented.  Everything in this module aims
//! to accomplish that goal.
//!

mod aggregation;
mod body;
mod collapsing;
mod condition;
mod criterion;
mod field;
mod geo_values;
mod numeric_value;
mod scalar_value;
mod search_trait;
mod sort_directive;
mod target;

pub use aggregation::*;
pub use body::*;
pub use collapsing::*;
pub use condition::*;
pub use criterion::*;
pub use field::*;
pub use geo_values::*;
pub use numeric_value::*;
pub use scalar_value::*;
pub use search_trait::*;
pub use sort_directive::*;
pub use target::*;

/// Used to construct a search request for Elasticsearch
#[derive(Debug, Clone, Default)]
pub struct Search {
    positive_criteria: Vec<Criterion>,
    negative_criteria: Vec<Criterion>,
    sorts: Vec<SortDirective>,
    aggregations: Option<AggCollection>,
    limit: Option<usize>,
    offset: Option<usize>,
    collapse_by: Option<CollapseBy>,
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

    fn sort_directives(&self) -> Option<&Vec<SortDirective>> {
        if self.sorts.is_empty() {
            None
        } else {
            Some(&self.sorts)
        }
    }

    fn aggregations(&self) -> Option<&AggCollection> {
        self.aggregations.as_ref()
    }

    fn collapse_by(&self) -> Option<&CollapseBy> {
        self.collapse_by.as_ref()
    }
}

impl CriteriaBuilder for Search {
    type Bucket = NormalBucket;

    fn positive_criteria_mut(&mut self) -> &mut Vec<Criterion> {
        &mut self.positive_criteria
    }

    fn negative_criteria_mut(&mut self) -> &mut Vec<Criterion> {
        &mut self.negative_criteria
    }
}

impl AggregationBuilder for Search {
    fn aggregations_mut(&mut self) -> &mut AggCollection {
        self.aggregations.get_or_insert_with(AggCollection::default)
    }
}

impl SortBuilderTrait for Search {
    fn sort_directives_mut(&mut self) -> &mut Vec<SortDirective> {
        &mut self.sorts
    }
}

impl CollapseBuilderTrait for Search {
    fn collapse_mut(&mut self) -> &mut Option<CollapseBy> {
        &mut self.collapse_by
    }
}
