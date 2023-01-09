use super::*;

/// Aides creating sub-aggreagtions for a filter aggregation as
/// well as finalizing the filter aggregation into the collection.
#[derive(Debug)]
pub struct AggBuilderWithSearch<'a, B: AggregationBuilder> {
    pub(super) name: Name,
    pub(super) search: AllMatch,
    pub(super) builder: &'a mut B,
    pub(super) sub_aggs: Option<AggCollection>,
}

impl<'a, B: AggregationBuilder> SubAggregationBuilder for AggBuilderWithSearch<'a, B> {
    fn maybe_sub_agg(&mut self) -> &mut Option<AggCollection> {
        &mut self.sub_aggs
    }
}

impl<'a, B: AggregationBuilder> Drop for AggBuilderWithSearch<'a, B> {
    fn drop(&mut self) {
        let mut name = "".into();
        let mut search = AllMatch::default();
        std::mem::swap(&mut name, &mut self.name);
        std::mem::swap(&mut search, &mut self.search);

        let agg = match self.sub_aggs.take() {
            Some(aggs) => FilterAggregation::new_with_sub_aggs(search, aggs),
            None => FilterAggregation::new(search),
        };

        self.builder.aggregations_mut().insert(name, agg);
    }
}
