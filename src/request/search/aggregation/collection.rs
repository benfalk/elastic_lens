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

impl AggCollection {
    /// adds an aggregation into the collection by name
    pub fn insert<N, A>(&mut self, name: N, agg: A)
    where
        N: Into<Name>,
        A: AggregationData,
    {
        self.data.insert(name.into(), agg.into());
    }

    /// returns true if there is no aggregations
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}

impl AggregationBuilder for AggCollection {
    fn aggregations_mut(&mut self) -> &mut AggCollection {
        self
    }
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
