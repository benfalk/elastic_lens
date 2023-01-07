use crate::request::search::*;
use std::marker::PhantomData;
use std::ops::Range;

use super::geo_distance_builder::UnitPicker;

/// Field Criteria Builder
///
/// Starts the construction of different criteria which can target
/// a field in Elasticsearch.  I realize the amount of "mental gymnastics"
/// going on with this builder is a bit over the top; however, it's the
/// best way I've found to ensure the right kind of bucket type tags are
/// propagated down the builder chain.
///
/// These ZST, zero sized type, tags aid in the ergonomic feel of jailing
/// certain experiences to where they make sense.
#[derive(Debug)]
#[must_use = "to add a field criteria this must be consumed"]
pub struct FieldCriteriaBuilder<'a, T: BucketPlacer, B: CriteriaBuilder> {
    builder: &'a mut B,
    field: Field,
    marker: PhantomData<T>,
}

impl<'a, T: BucketPlacer, B: CriteriaBuilder> FieldCriteriaBuilder<'a, T, B> {
    pub(super) fn new(builder: &'a mut B, field: Field) -> Self {
        Self {
            builder,
            field,
            marker: PhantomData,
        }
    }

    /// Is true if the value provided is found on the field.
    ///
    /// While some fields on your document may only have one value in
    /// Elasticsearch it's actually an index of values; which may just
    /// be one in that case.  You can also think of this as `equals`
    /// for fields that fall into this category.
    pub fn contains<V: Into<ScalarValue>>(self, value: V) {
        let term_filter = TermFilter::single(self.field, value);
        T::push(self.builder, term_filter);
    }

    /// Select if any of the provided values are found on the field
    pub fn any_of<V, S>(self, values: V)
    where
        S: Into<ScalarValue>,
        V: IntoIterator<Item = S>,
    {
        let terms = TermFilter::many(self.field, values);
        T::push(self.builder, terms);
    }

    /// Selects if the field has any values
    pub fn exists(self) {
        let exists = ExistsFilter::new(self.field);
        T::push(self.builder, exists)
    }

    /// Select when values are less than value
    pub fn less_than<V: Into<ScalarValue>>(self, value: V) {
        let range = RangeFilterBuilder::new(self.field).less_than(value).build();
        T::push(self.builder, range)
    }

    /// Select when values are less than OR equal to value
    pub fn less_than_or_equal<V: Into<ScalarValue>>(self, value: V) {
        let range = RangeFilterBuilder::new(self.field)
            .less_than_or_equal(value)
            .build();
        T::push(self.builder, range)
    }

    /// Select when values are greater than value
    pub fn greater_than<V: Into<ScalarValue>>(self, value: V) {
        let range = RangeFilterBuilder::new(self.field)
            .greater_than(value)
            .build();
        T::push(self.builder, range)
    }

    /// Select when values are greater than OR equal to value
    pub fn greater_than_or_equal<V: Into<ScalarValue>>(self, value: V) {
        let range = RangeFilterBuilder::new(self.field)
            .greater_than_or_equal(value)
            .build();
        T::push(self.builder, range)
    }

    /// Select when values are between the range inclusively
    pub fn between<V>(self, value: Range<V>)
    where
        V: PartialOrd + Into<ScalarValue>,
    {
        let range = RangeFilterBuilder::new(self.field).between(value).build();
        T::push(self.builder, range)
    }

    /// the distance without a unit type applied yet, the next
    /// segment in this chain is to select a unit of measurement.
    pub fn within(self, distance: usize) -> UnitPicker<'a, T, B> {
        UnitPicker {
            marker: PhantomData,
            builder: self.builder,
            field: self.field,
            amount: distance,
        }
    }
}

impl<'a, B: CriteriaBuilder> FieldCriteriaBuilder<'a, PositiveBucket, B> {
    /// changes the condition from a selection to an exclusion
    #[allow(clippy::should_implement_trait)]
    pub fn not(self) -> FieldCriteriaBuilder<'a, NegativeBucket, B> {
        FieldCriteriaBuilder {
            builder: self.builder,
            field: self.field,
            marker: PhantomData,
        }
    }
}

impl<'a, B: CriteriaBuilder> FieldCriteriaBuilder<'a, GroupedOrBucket, B> {
    /// changes the condition from a selection to an exclusion
    #[allow(clippy::should_implement_trait)]
    pub fn not(self) -> FieldCriteriaBuilder<'a, NegativeGroupedOrBucket, B> {
        FieldCriteriaBuilder {
            builder: self.builder,
            field: self.field,
            marker: PhantomData,
        }
    }
}
