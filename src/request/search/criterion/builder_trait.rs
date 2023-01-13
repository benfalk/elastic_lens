use super::*;
mod buckets;
pub use buckets::*;

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

    /// Having the provided condition
    fn with<SC: Into<SearchCondition>>(&mut self, condition: SC) {
        let condition = condition.into();
        <Self::Bucket as BucketPlacer>::push(self, condition.criterion, condition.tag);
    }
}
