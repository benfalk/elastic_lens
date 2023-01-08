use super::*;

/// This trait is what allows for the construction of aggregations
/// for not only the root search; but also any aggregations that
/// support sub-aggregations.  If you want to have your own Agg
/// builder the only method you need to implement is `aggregations_mut`
pub trait AggregationBuilder: Sized {
    /// mutable reference to the agg collection to work from
    fn aggregations_mut(&mut self) -> &mut AggCollection;
}
