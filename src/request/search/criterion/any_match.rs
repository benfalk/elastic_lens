use super::*;
use serde::Serialize;

/// Selects if any of the criteria given match
pub fn if_any_match<F>(mut func: F) -> AnyMatch
where
    F: FnMut(&mut AnyMatch),
{
    let mut any = AnyMatch::default();
    func(&mut any);
    any
}

/// A grouped set of criteria where only one
/// has to match for the document to be selected
#[derive(Debug, Clone, Default)]
pub struct AnyMatch {
    criteria: Vec<Criterion>,
}

impl AnyMatch {
    /// Create a new group of OR'ed criteria where
    /// with a preset capacity to avoid excess allocations
    pub fn with_capacity(capacity: usize) -> Self {
        AnyMatch {
            criteria: Vec::with_capacity(capacity),
        }
    }

    /// Add a criterion to the collection
    pub fn add<C: Into<Criterion>>(&mut self, criterion: C) {
        self.criteria.push(criterion.into());
    }

    pub(crate) fn has_data(&self) -> bool {
        !self.criteria.is_empty()
    }
}

impl Serialize for AnyMatch {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let node = ElasticsearchQuery {
            bool: ElasticsearchBool {
                filter: None,
                must_not: None,
                should: Some(&self.criteria),
            },
        };

        node.serialize(serializer)
    }
}

impl From<AnyMatch> for Criterion {
    fn from(value: AnyMatch) -> Self {
        Self::AnyMatch(value)
    }
}

impl CriterionData for AnyMatch {}

impl CriteriaBuilder for AnyMatch {
    type Bucket = GroupedOrBucket;

    fn positive_criteria_mut(&mut self) -> &mut Vec<Criterion> {
        &mut self.criteria
    }

    fn negative_criteria_mut(&mut self) -> &mut Vec<Criterion> {
        unreachable!("should never work with a negative bucket for AnyMatch")
    }
}
