use super::*;

mod buckets;
mod field_criteria_builder;
mod geo_distance_builder;

pub use buckets::*;
pub use field_criteria_builder::*;

/// This is the work-horse of building critera for a search.  The
/// only two methods that are required are `positive_criteria_mut`
/// and `negative_criteria_mut`.  The Bucket must also be set to
/// drive what level of functionality it is currently in.  Unless
/// you know what you're doing I suggest setting it to `PositiveBucket`.
pub trait CriteriaBuilder: Sized {
    /// This drives what functionality is available to use.  See the
    /// `BucketPlacer` trait for more information on it's use.
    type Bucket: BucketPlacer;

    /// Should return a mutable reference to a vector of mutable
    /// criteria.  These are criteria which select when true.
    fn positive_criteria_mut(&mut self) -> &mut Vec<Criterion>;

    /// Should return a mutable reference to a vector of mutable
    /// criteria.  These are criteria which exclude when true.
    fn negative_criteria_mut(&mut self) -> &mut Vec<Criterion>;

    /// Begin construction of a criterion targeting a field
    fn field<F>(&mut self, field: F) -> FieldCriteriaBuilder<'_, Self::Bucket, Self>
    where
        F: Into<Field>,
        Self::Bucket: BucketPlacer,
    {
        FieldCriteriaBuilder::new(self, field.into())
    }

    /// Selects if any of the created conditions is true
    fn if_any_match<F>(&mut self, mut func: F)
    where
        F: FnMut(&mut AnyMatch),
    {
        let mut any_match = AnyMatch::default();
        func(&mut any_match);
        <Self::Bucket as BucketPlacer>::push(self, any_match);
    }
}
