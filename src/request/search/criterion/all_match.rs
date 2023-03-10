use super::*;
use serde::Serialize;

/// All Match
///
/// This functions like a root bool node and
/// groups similar positive and negative criteria
/// together.  It's used to group conditions together
/// in an `AnyMatch` as well as the primary way to
/// create stand-alone criteria to apply to different
/// searches.
#[derive(Debug, Clone, Default)]
pub struct AllMatch {
    positive_criteria: Vec<Criterion>,
    negative_criteria: Vec<Criterion>,
}

impl AllMatch {
    pub(crate) fn has_data(&self) -> bool {
        !(self.negative_criteria.is_empty() && self.positive_criteria.is_empty())
    }
}

/// Selects if all criteria given select
pub fn if_all_match<F>(mut func: F) -> AllMatch
where
    F: FnMut(&mut AllMatch),
{
    let mut all_match = AllMatch::default();
    func(&mut all_match);
    all_match
}

impl Serialize for AllMatch {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let filter = if self.positive_criteria.is_empty() {
            None
        } else {
            Some(&self.positive_criteria)
        };

        let must_not = if self.negative_criteria.is_empty() {
            None
        } else {
            Some(&self.negative_criteria)
        };

        let node = ElasticsearchQuery {
            bool: ElasticsearchBool {
                filter,
                must_not,
                should: None,
            },
        };

        node.serialize(serializer)
    }
}

impl From<AllMatch> for Criterion {
    fn from(value: AllMatch) -> Self {
        Self::AllMatch(value)
    }
}

impl CriterionData for AllMatch {}

impl CriteriaBuilder for AllMatch {
    type Bucket = NormalBucket;

    fn positive_criteria_mut(&mut self) -> &mut Vec<Criterion> {
        &mut self.positive_criteria
    }

    fn negative_criteria_mut(&mut self) -> &mut Vec<Criterion> {
        &mut self.negative_criteria
    }
}
