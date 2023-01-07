use super::*;
use serde::Serialize;

/// Set of criteria that all get their own grouped
/// negation.  Normally handy when in an OR block,
/// but can also be handy if you are creating criteria
/// separately to be added later to a search.
#[derive(Debug, Clone, Default)]
pub struct NotAll {
    criteria: Vec<Criterion>,
}

impl NotAll {
    /// Consumes a criterion and negates it
    pub fn single<C: Into<Criterion>>(criterion: C) -> Self {
        Self {
            criteria: vec![criterion.into()],
        }
    }

    /// adds a criterion to be negated
    pub fn add<C: Into<Criterion>>(&mut self, criterion: C) {
        self.criteria.push(criterion.into());
    }

    /// create an empty group with a pre-set capacity
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            criteria: Vec::with_capacity(capacity),
        }
    }
}

impl Serialize for NotAll {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let node = ElasticsearchQuery {
            bool: ElasticsearchBool {
                must_not: Some(&self.criteria),
                filter: None,
                should: None,
            },
        };

        node.serialize(serializer)
    }
}

impl From<NotAll> for Criterion {
    fn from(value: NotAll) -> Self {
        Self::NotAll(value)
    }
}

impl CriterionData for NotAll {}
