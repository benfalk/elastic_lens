use std::marker::PhantomData;
use crate::request::search::*;

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

impl <'a, T: BucketPlacer, B: CriteriaBuilder> FieldCriteriaBuilder<'a, T, B> {
    pub(super) fn new(builder: &'a mut B, field: Field) -> Self {
        Self {
            builder,
            field,
            marker: PhantomData
        }
    }

    /// Is true if the value provided is found on the field.
    ///
    /// While some fields on your document may only have one value in
    /// Elasticsearch it's actually an index of values; which may just
    /// be one in that case.  You can also think of this as `equals`
    /// for fields that fall into this category.
    pub fn contains<V: Into<ScalarValue>>(self, value: V) {
        let term_filter = TermFilter::new(self.field, value.into());
        T::push(self.builder, term_filter.into());
    }
}

impl <'a, B: CriteriaBuilder> FieldCriteriaBuilder<'a, PositiveBucket, B> {
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
