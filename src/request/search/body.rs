use crate::client::Settings;

use super::*;
use serde::Serialize;

impl<'a, S: SearchTrait> From<&'a S> for SearchBody<'a> {
    fn from(value: &'a S) -> Self {
        Self {
            size: value.limit(),
            from: value.offset(),
            query: ElasticsearchQuery {
                bool: ElasticsearchBool {
                    filter: value.positive_criteria(),
                    must_not: value.negative_criteria(),
                    should: None,
                },
            },
            aggs: value.aggregations(),
        }
    }
}

impl<'a> SearchBody<'a> {
    /// Apply default values for a serach if any
    pub(crate) fn apply_defaults(&mut self, settings: &Settings) {
        if self.size.is_none() && settings.default_limit.is_some() {
            self.size = settings.default_limit;
        }
    }
}

#[derive(Debug, Serialize)]
#[doc(hidden)]
pub struct SearchBody<'a> {
    #[serde(skip_serializing_if = "SkipNode::not_needed")]
    size: Option<usize>,

    #[serde(skip_serializing_if = "SkipNode::not_needed")]
    from: Option<usize>,

    #[serde(skip_serializing_if = "SkipNode::not_needed")]
    query: ElasticsearchQuery<'a>,

    #[serde(skip_serializing_if = "SkipNode::not_needed")]
    aggs: Option<&'a AggCollection>,
}

#[derive(Debug, Serialize)]
#[doc(hidden)]
pub struct ElasticsearchQuery<'a> {
    #[serde(skip_serializing_if = "SkipNode::not_needed")]
    pub(super) bool: ElasticsearchBool<'a>,
}

#[derive(Debug, Serialize)]
#[doc(hidden)]
pub struct ElasticsearchBool<'a> {
    #[serde(skip_serializing_if = "SkipNode::not_needed")]
    pub(super) filter: Option<&'a Vec<Criterion>>,

    #[serde(skip_serializing_if = "SkipNode::not_needed")]
    pub(super) must_not: Option<&'a Vec<Criterion>>,

    #[serde(skip_serializing_if = "SkipNode::not_needed")]
    pub(super) should: Option<&'a Vec<Criterion>>,
}

trait SkipNode {
    fn is_needed(&self) -> bool;

    fn not_needed(&self) -> bool {
        !self.is_needed()
    }
}

impl<T> SkipNode for Option<T> {
    fn is_needed(&self) -> bool {
        self.is_some()
    }
}

impl<'a> SkipNode for ElasticsearchBool<'a> {
    fn is_needed(&self) -> bool {
        if self.should.is_needed() {
            return true;
        }

        if self.filter.is_needed() {
            return true;
        }

        if self.must_not.is_needed() {
            return true;
        }

        false
    }
}

impl<'a> SkipNode for ElasticsearchQuery<'a> {
    fn is_needed(&self) -> bool {
        self.bool.is_needed()
    }
}
