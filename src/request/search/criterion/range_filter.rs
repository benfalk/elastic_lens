use super::*;
use serde::{ser::SerializeMap, Serialize};
use std::ops::Range;

/// Range Field
///
/// Selects based on where a field falls inside either
/// an open-ended or closed range.
///
#[derive(Debug, Clone)]
pub struct RangeFilter {
    field: Field,
    upper_bound: Option<UpperBound>,
    lower_bound: Option<LowerBound>,
}

/// Build a range criterion
#[derive(Debug, Clone)]
pub struct RangeFilterBuilder {
    field: Field,
    upper_bound: Option<UpperBound>,
    lower_bound: Option<LowerBound>,
}

impl RangeFilterBuilder {
    /// Start construction of a range fiilter with the only
    /// required param, a Field
    pub fn new<F: Into<Field>>(field: F) -> Self {
        Self {
            field: field.into(),
            upper_bound: None,
            lower_bound: None,
        }
    }

    /// Select when values are greater than value
    pub fn greater_than<V: Into<ScalarValue>>(mut self, value: V) -> Self {
        self.lower_bound = Some(LowerBound::GreaterThan(value.into()));
        self
    }

    /// Select when values are greater than OR equal to value
    pub fn greater_than_or_equal<V: Into<ScalarValue>>(mut self, value: V) -> Self {
        self.lower_bound = Some(LowerBound::GreaterThanOrEqual(value.into()));
        self
    }

    /// Select when values are less than value
    pub fn less_than<V: Into<ScalarValue>>(mut self, value: V) -> Self {
        self.upper_bound = Some(UpperBound::LessThan(value.into()));
        self
    }

    /// Select when values are less than OR equal to value
    pub fn less_than_or_equal<V: Into<ScalarValue>>(mut self, value: V) -> Self {
        self.upper_bound = Some(UpperBound::LessThanOrEqual(value.into()));
        self
    }

    /// Select when values are between the range inclusively
    pub fn between<V>(mut self, value: Range<V>) -> Self
    where
        V: PartialOrd + Into<ScalarValue>,
    {
        self.upper_bound = Some(UpperBound::LessThanOrEqual(value.end.into()));
        self.lower_bound = Some(LowerBound::GreaterThanOrEqual(value.start.into()));
        self
    }

    /// Creates RangeFilter
    pub fn build(self) -> RangeFilter {
        RangeFilter {
            field: self.field,
            upper_bound: self.upper_bound,
            lower_bound: self.lower_bound,
        }
    }
}

#[derive(Debug, Clone)]
enum LowerBound {
    GreaterThan(ScalarValue),
    GreaterThanOrEqual(ScalarValue),
}

#[derive(Debug, Clone)]
enum UpperBound {
    LessThan(ScalarValue),
    LessThanOrEqual(ScalarValue),
}

impl LowerBound {
    fn apply_entry<M>(&self, map: &mut M) -> Result<(), <M as SerializeMap>::Error>
    where
        M: SerializeMap,
    {
        match self {
            Self::GreaterThan(value) => map.serialize_entry("gt", value),
            Self::GreaterThanOrEqual(value) => map.serialize_entry("gte", value),
        }
    }
}

impl UpperBound {
    fn apply_entry<M>(&self, map: &mut M) -> Result<(), <M as SerializeMap>::Error>
    where
        M: SerializeMap,
    {
        match self {
            Self::LessThan(value) => map.serialize_entry("lt", value),
            Self::LessThanOrEqual(value) => map.serialize_entry("lte", value),
        }
    }
}

impl CriterionData for RangeFilter {}

impl From<RangeFilter> for Criterion {
    fn from(value: RangeFilter) -> Self {
        Self::Range(value)
    }
}

impl Serialize for RangeFilter {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        struct Params<'a> {
            range: &'a RangeFilter,
        }

        impl<'a> Serialize for Params<'a> {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                let mut size = 0;

                if self.range.upper_bound.is_some() {
                    size += 1;
                }

                if self.range.lower_bound.is_some() {
                    size += 1;
                }

                let mut map = serializer.serialize_map(Some(size))?;

                if let Some(range) = &self.range.lower_bound {
                    range.apply_entry(&mut map)?;
                }

                if let Some(range) = &self.range.upper_bound {
                    range.apply_entry(&mut map)?;
                }

                map.end()
            }
        }

        struct FieldContainer<'a> {
            range: &'a RangeFilter,
        }

        impl<'a> Serialize for FieldContainer<'a> {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                let params = Params { range: self.range };
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_entry(&self.range.field, &params)?;
                map.end()
            }
        }

        let container = FieldContainer { range: self };
        let mut map = serializer.serialize_map(Some(1))?;
        map.serialize_entry("range", &container)?;
        map.end()
    }
}
