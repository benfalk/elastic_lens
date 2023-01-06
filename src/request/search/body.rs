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
                },
            },
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
}

#[derive(Debug, Serialize)]
#[doc(hidden)]
pub struct ElasticsearchQuery<'a> {
    #[serde(skip_serializing_if = "SkipNode::not_needed")]
    bool: ElasticsearchBool<'a>,
}

#[derive(Debug, Serialize)]
#[doc(hidden)]
pub struct ElasticsearchBool<'a> {
    #[serde(skip_serializing_if = "SkipNode::not_needed")]
    filter: Option<&'a Vec<Criterion>>,

    #[serde(skip_serializing_if = "SkipNode::not_needed")]
    must_not: Option<&'a Vec<Criterion>>,
}

trait SkipNode {
    fn not_needed(&self) -> bool;
}

impl<T> SkipNode for Option<T> {
    fn not_needed(&self) -> bool {
        self.is_none()
    }
}

impl<'a> SkipNode for ElasticsearchBool<'a> {
    fn not_needed(&self) -> bool {
        self.filter.not_needed() && self.must_not.not_needed()
    }
}

impl<'a> SkipNode for ElasticsearchQuery<'a> {
    fn not_needed(&self) -> bool {
        self.bool.not_needed()
    }
}
