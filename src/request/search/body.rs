use crate::client::Settings;

use super::*;
use serde::Serialize;

impl<'a, S: SearchTrait> From<&'a S> for SearchBody<'a> {
    fn from(value: &'a S) -> Self {
        Self {
            size: value.limit(),
            from: value.offset(),
            query: determine_root(value),
            aggs: value.aggregations(),
            sort: determine_sorts(value),
            collapse: value.collapse_by(),
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

#[derive(Debug)]
#[doc(hidden)]
pub enum QueryRoot<'a> {
    RootQuery(ElasticsearchQuery<'a>),
    ScriptScore(ScriptScoreRoot<'a>),
}

#[derive(Debug, Serialize)]
#[doc(hidden)]
pub struct ScriptScoreRoot<'a> {
    script_score: ScriptScoreValues<'a>,
}

#[derive(Debug, Serialize)]
#[doc(hidden)]
pub struct ScriptScoreValues<'a> {
    #[serde(skip_serializing_if = "SkipNode::not_needed")]
    query: ElasticsearchQuery<'a>,
    script: &'a SortDirective,
}

#[derive(Debug, Serialize)]
#[doc(hidden)]
pub struct SearchBody<'a> {
    #[serde(skip_serializing_if = "SkipNode::not_needed")]
    size: Option<usize>,

    #[serde(skip_serializing_if = "SkipNode::not_needed")]
    from: Option<usize>,

    #[serde(skip_serializing_if = "SkipNode::not_needed")]
    query: QueryRoot<'a>,

    #[serde(skip_serializing_if = "SkipNode::not_needed")]
    aggs: Option<&'a AggCollection>,

    #[serde(skip_serializing_if = "SkipNode::not_needed")]
    sort: Option<&'a Vec<SortDirective>>,

    #[serde(skip_serializing_if = "SkipNode::not_needed")]
    collapse: Option<&'a CollapseBy>,
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

fn determine_root<S: SearchTrait>(search: &S) -> QueryRoot<'_> {
    if has_a_script_sort(search) {
        QueryRoot::ScriptScore(ScriptScoreRoot {
            script_score: ScriptScoreValues {
                query: ElasticsearchQuery {
                    bool: ElasticsearchBool {
                        filter: search.positive_criteria(),
                        must_not: search.negative_criteria(),
                        should: None,
                    },
                },
                script: fetch_script(search),
            },
        })
    } else {
        QueryRoot::RootQuery(ElasticsearchQuery {
            bool: ElasticsearchBool {
                filter: search.positive_criteria(),
                must_not: search.negative_criteria(),
                should: None,
            },
        })
    }
}

fn determine_sorts<S: SearchTrait>(search: &S) -> Option<&Vec<SortDirective>> {
    if has_a_script_sort(search) {
        None
    } else {
        search.sort_directives()
    }
}

fn has_a_script_sort<S: SearchTrait>(search: &S) -> bool {
    match search.sort_directives() {
        None => false,
        Some(sorts) => sorts.iter().any(SortDirective::is_script_score),
    }
}

fn fetch_script<S: SearchTrait>(search: &S) -> &SortDirective {
    search
        .sort_directives()
        .unwrap()
        .iter()
        .find(|sort| sort.is_script_score())
        .unwrap()
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

impl<'a> SkipNode for QueryRoot<'a> {
    fn is_needed(&self) -> bool {
        match self {
            Self::ScriptScore(_) => true,
            Self::RootQuery(quey) => quey.is_needed(),
        }
    }
}

impl<'a> Serialize for QueryRoot<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Self::RootQuery(body) => body.serialize(serializer),
            Self::ScriptScore(body) => body.serialize(serializer),
        }
    }
}
