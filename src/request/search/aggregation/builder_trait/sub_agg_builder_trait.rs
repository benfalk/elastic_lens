use super::*;

/// Aids in the construction of sub-aggregations.
///
/// Elasticsearch has support for different kinds of aggregations
/// to be assigned under an aggregation in a nested fashion.  For
/// example `TermsAggregation` can have `StatsAggregations` under
/// it to report some stats for each of the top terms.  These
/// extra dimensions of data are useful for reporting and building
/// strategies for what filters can be applied next to a search.
pub trait SubAggregationBuilder {
    /// return an optional aggregation collection, because not all
    /// aggregations have sub aggregations if if they support it
    fn maybe_sub_agg(&mut self) -> &mut Option<AggCollection>;

    /// Add sub aggregations to an aggreagtion node
    fn with_sub_aggregations<F>(&mut self, mut func: F)
    where
        F: FnMut(&mut AggCollection),
    {
        let sub_aggs = self
            .maybe_sub_agg()
            .get_or_insert_with(AggCollection::default);

        func(sub_aggs);

        if sub_aggs.is_empty() {
            self.maybe_sub_agg().take();
        }
    }
}
