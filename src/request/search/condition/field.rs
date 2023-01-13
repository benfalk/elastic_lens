use super::*;
use std::ops::Range;

/// A field to target on the Elasticsearch index
pub fn field<F: Into<Field>>(field: F) -> Field {
    field.into()
}

impl Field {
    /// Select if field has this value
    pub fn contains<S: Into<ScalarValue>>(&self, value: S) -> SearchCondition {
        SearchCondition::from(TermFilter::single(self.clone(), value))
    }

    /// Select when values are greater than value
    pub fn greater_than<V: Into<ScalarValue>>(&self, value: V) -> RangeFilterBuilder {
        RangeFilterBuilder::new(self.clone()).greater_than(value)
    }

    /// Select when values are greater than OR equal to value
    pub fn greater_than_or_equal<V: Into<ScalarValue>>(&self, value: V) -> RangeFilterBuilder {
        RangeFilterBuilder::new(self.clone()).greater_than_or_equal(value)
    }

    /// Select when values are less than value
    pub fn less_than<V: Into<ScalarValue>>(&self, value: V) -> RangeFilterBuilder {
        RangeFilterBuilder::new(self.clone()).less_than(value)
    }

    /// Select when values are less than OR equal to value
    pub fn less_than_or_equal<V: Into<ScalarValue>>(&self, value: V) -> RangeFilterBuilder {
        RangeFilterBuilder::new(self.clone()).less_than_or_equal(value)
    }

    /// Select when values are between the range inclusively
    pub fn between<V>(&self, value: Range<V>) -> RangeFilterBuilder
    where
        V: PartialOrd + Into<ScalarValue>,
    {
        RangeFilterBuilder::new(self.clone()).between(value)
    }

    /// Selects if the field exists
    pub fn exists(&self) -> SearchCondition {
        ExistsFilter::new(self.clone()).into()
    }

    /// Starts a new distance filter
    pub fn within(&self, unit: usize) -> GeoDistanceFilterBuilder<2> {
        GeoDistanceFilterBuilder::for_field(self.clone()).within(unit)
    }

    /// Select if the condition has at least one of the values
    /// provided.  This is better than many `contains` fitlers
    /// in an `if_any_match` statement.
    pub fn has_any_of<V, S>(&self, values: V) -> SearchCondition
    where
        S: Into<ScalarValue>,
        V: IntoIterator<Item = S>,
    {
        TermFilter::many(self.clone(), values).into()
    }
}
