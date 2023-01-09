use super::*;

mod filter_agg_builder;
mod stats_builder;
mod sub_agg_builder_trait;
mod terms_builder;

pub use filter_agg_builder::*;
pub use stats_builder::*;
pub use sub_agg_builder_trait::*;
pub use terms_builder::*;

/// This trait is what allows for the construction of aggregations
/// for not only the root search; but also any aggregations that
/// support sub-aggregations.  If you want to have your own Agg
/// builder the only method you need to implement is `aggregations_mut`
pub trait AggregationBuilder: Sized {
    /// mutable reference to the agg collection to work from
    fn aggregations_mut(&mut self) -> &mut AggCollection;

    /// start construction of an aggregation by supplying a name
    fn create_aggregation<N: Into<Name>>(&mut self, name: N) -> AggBuilderWithName<'_, Self> {
        AggBuilderWithName {
            name: name.into(),
            builder: self,
        }
    }
}

/// Begins construction of an aggregation with it's name
#[derive(Debug)]
#[must_use = "this must be consumed to finish constructing an aggregation"]
pub struct AggBuilderWithName<'a, B: AggregationBuilder> {
    name: Name,
    builder: &'a mut B,
}

/// Continues construction targeting a field
#[derive(Debug)]
#[must_use = "this must be consumed to finish constructing an aggregation"]
pub struct AggBuilderWithField<'a, B: AggregationBuilder> {
    name: Name,
    builder: &'a mut B,
    field: Field,
}

impl<'a, B: AggregationBuilder> AggBuilderWithName<'a, B> {
    /// Starts construction of a filter aggregation; which is a
    /// single bucket aggregation that narrows the set of documents
    /// based on the provided search
    pub fn filtered_by<F>(self, mut func: F) -> AggBuilderWithSearch<'a, B>
    where
        F: FnMut(&mut AllMatch),
    {
        let mut search = AllMatch::default();

        func(&mut search);

        AggBuilderWithSearch {
            name: self.name,
            search,
            builder: self.builder,
            sub_aggs: None,
        }
    }

    /// targets a field for aggregation
    pub fn for_field<F: Into<Field>>(self, field: F) -> AggBuilderWithField<'a, B> {
        AggBuilderWithField {
            name: self.name,
            builder: self.builder,
            field: field.into(),
        }
    }
}

impl<'a, B: AggregationBuilder> AggBuilderWithField<'a, B> {
    /// Constructs an aggregation that will count the top
    /// terms of the given field.
    pub fn count_terms(self) -> AggTermsBuider<'a, B> {
        AggTermsBuider {
            name: self.name,
            field: self.field,
            size: None,
            sub_aggs: None,
            builder: self.builder,
        }
    }

    /// Collect stats for the given field.  You may
    /// also apply a value to use when a document is
    /// missing a value for the field.
    pub fn collect_stats(self) -> AggStatsBuilder<'a, B> {
        AggStatsBuilder {
            name: self.name,
            field: self.field,
            missing_value: None,
            builder: self.builder,
        }
    }
}
