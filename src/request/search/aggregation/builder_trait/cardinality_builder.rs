use super::*;

/// Creates cardinality count aggregate for the given field.
#[derive(Debug)]
pub struct AggCardinalityBuilder<'a, B: AggregationBuilder> {
    pub(super) name: Name,
    pub(super) field: Field,
    pub(super) missing_value: Option<ScalarValue>,
    pub(super) precision_threshold: Option<u32>,
    pub(super) builder: &'a mut B,
}

impl<'a, B: AggregationBuilder> AggCardinalityBuilder<'a, B> {
    /// Documents which are missing a value are normally
    /// not counted; this allows you to specifiy what
    /// value to use when calculating cardinality
    /// when this happens.
    pub fn use_for_missing_value<V>(mut self, value: V) -> Self
    where
        V: Into<ScalarValue>,
    {
        self.missing_value = Some(value.into());
        self
    }

    /// Sets the precision threshold for the cardinality aggregation.
    /// The maximum allowed value is 40000.
    pub fn with_precision_threshold(mut self, threshold: u32) -> Self {
        self.precision_threshold = Some(threshold);
        self
    }
}

impl<'a, B: AggregationBuilder> Drop for AggCardinalityBuilder<'a, B> {
    fn drop(&mut self) {
        // Zero Allocation swap. Seems more sain than
        // dealing with Option<> everywhere...
        let mut name = "".into();
        let mut field = "".into();
        std::mem::swap(&mut name, &mut self.name);
        std::mem::swap(&mut field, &mut self.field);

        let mut agg = CardinalityAggregation::new(field);

        if let Some(value) = self.missing_value.take() {
            agg = agg.use_for_missing_value(value);
        }

        if let Some(threshold) = self.precision_threshold {
            agg = agg.with_precision_threshold(threshold);
        }

        self.builder.aggregations_mut().insert(name, agg);
    }
}
