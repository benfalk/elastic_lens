use super::*;
use serde::Serialize;

/// Nested Filter
///
///
#[derive(Debug, Clone)]
pub struct NestedFilter {
    criteria: AllMatch,
    path: Field,
}

impl NestedFilter {
    pub(crate) fn has_data(&self) -> bool {
        self.criteria.has_data()
    }
}

/// Selects with nested criteria
pub fn nested<F, FN>(field: F, mut func: FN) -> NestedFilter
where
    F: Into<Field>,
    FN: FnMut(&mut NestedFilter),
{
    let mut nested = NestedFilter {
        path: field.into(),
        criteria: Default::default(),
    };
    func(&mut nested);
    nested
}

impl CriteriaBuilder for NestedFilter {
    type Bucket = NormalBucket;

    fn positive_criteria_mut(&mut self) -> &mut Vec<Criterion> {
        self.criteria.positive_criteria_mut()
    }

    fn negative_criteria_mut(&mut self) -> &mut Vec<Criterion> {
        self.criteria.negative_criteria_mut()
    }
}

impl From<NestedFilter> for Criterion {
    fn from(value: NestedFilter) -> Self {
        Self::Nested(value)
    }
}

impl Serialize for NestedFilter {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        #[derive(Serialize)]
        struct Parts<'a> {
            query: &'a AllMatch,
            path: &'a Field,
        }

        #[derive(Serialize)]
        struct NestedAst<'a> {
            nested: Parts<'a>,
        }

        let nested = NestedAst {
            nested: Parts {
                path: &self.path,
                query: &self.criteria,
            },
        };

        nested.serialize(serializer)
    }
}
