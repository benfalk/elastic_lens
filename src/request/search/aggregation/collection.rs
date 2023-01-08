use super::*;
use serde::Serialize;
use std::collections::BTreeMap;

/// Grouped collection of `Aggregation` keyed
/// by `Name`.  This can be found on the root
/// of a `Body` request as well as a sub-aggregation
/// on different variants of other aggregations.
#[derive(Debug, Clone, Default)]
pub struct AggCollection {
    data: BTreeMap<Name, Aggregation>,
}

impl AggCollection {}

impl Serialize for AggCollection {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.data.serialize(serializer)
    }
}
