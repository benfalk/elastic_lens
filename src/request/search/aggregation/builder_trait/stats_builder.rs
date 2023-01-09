use super::*;

/// Creates stats aggregations for the given field.
#[derive(Debug)]
pub struct AggStatsBuilder<'a, B: AggregationBuilder> {
    pub(super) name: Name,
    pub(super) field: Field,
    pub(super) missing_value: Option<NumericValue>,
    pub(super) builder: &'a mut B,
}

impl<'a, B: AggregationBuilder> AggStatsBuilder<'a, B> {
    /// Document which are missing a value are normally
    /// not counted; this allows you to specifiy what
    /// value to use when calculating stats if a document
    /// when this happens.
    pub fn use_for_missing_value<N>(mut self, value: N)
    where
        N: Into<NumericValue>,
    {
        self.missing_value = Some(value.into());
    }
}

impl<'a, B: AggregationBuilder> Drop for AggStatsBuilder<'a, B> {
    fn drop(&mut self) {
        // Zero Allocation swap. Seems more sain than
        // dealing with Option<> everywhere...
        let mut name = "".into();
        let mut field = "".into();
        std::mem::swap(&mut name, &mut self.name);
        std::mem::swap(&mut field, &mut self.field);

        let agg = match self.missing_value {
            Some(value) => StatsAggregation::new_with_missing(field, value),
            None => StatsAggregation::new(field),
        };

        self.builder.aggregations_mut().insert(name, agg);
    }
}
