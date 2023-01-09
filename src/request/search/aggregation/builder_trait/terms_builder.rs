use super::*;

/// Aids in the construction of terms aggregations.  This
/// utilizes the `Drop` trait to finish construction; which
/// allows for optional settings to be assigned.
#[derive(Debug)]
pub struct AggTermsBuider<'a, B: AggregationBuilder> {
    pub(super) name: Name,
    pub(super) field: Field,
    pub(super) size: Option<u16>,
    pub(super) builder: &'a mut B,
    pub(super) sub_aggs: Option<AggCollection>,
}

impl<'a, B: AggregationBuilder> AggTermsBuider<'a, B> {
    /// By default, the terms aggregation returns the top ten terms
    /// with the most documents. Use the size parameter to return more
    /// terms, up to the `search.max_buckets` limit set in Elasticsearch.
    pub fn for_top(&mut self, buckets: u16) -> &mut Self {
        self.size = Some(buckets);
        self
    }
}

impl<'a, B: AggregationBuilder> SubAggregationBuilder for AggTermsBuider<'a, B> {
    fn maybe_sub_agg(&mut self) -> &mut Option<AggCollection> {
        &mut self.sub_aggs
    }
}

impl<'a, B: AggregationBuilder> Drop for AggTermsBuider<'a, B> {
    fn drop(&mut self) {
        // Zero Allocation swap. Seems more sain than
        // dealing with Option<> everywhere...
        let mut name = "".into();
        let mut field = "".into();
        std::mem::swap(&mut name, &mut self.name);
        std::mem::swap(&mut field, &mut self.field);

        let mut agg = match self.size {
            Some(size) => TermsAggregation::new_with_size(field, size),
            None => TermsAggregation::new(field),
        };

        agg.sub_aggs = self.sub_aggs.take();

        self.builder.aggregations_mut().insert(name, agg);
    }
}
