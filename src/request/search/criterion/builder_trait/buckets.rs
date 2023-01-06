use super::*;

/// ZST trait tag to decide where criteria are placed for a builder
pub trait BucketPlacer {
    /// place the criterion where it makes sense to
    fn push(builder: &mut impl CriteriaBuilder, criterion: Criterion);
}

/// ZST struct which will place criteria into the positive
/// bucket.  This also helps to switch to a "negative" bucket
/// when in positive mode and avoid nested "nots".
#[derive(Debug, Copy, Clone)]
pub struct PositiveBucket {}

impl BucketPlacer for PositiveBucket {
    fn push(builder: &mut impl CriteriaBuilder, criterion: Criterion) {
        builder.positive_criteria_mut().push(criterion);
    }
}

/// ZST struct which places criteria into the negative bucket.
#[derive(Debug, Copy, Clone)]
pub struct NegativeBucket {}

impl BucketPlacer for NegativeBucket {
    fn push(builder: &mut impl CriteriaBuilder, criterion: Criterion) {
        builder.negative_criteria_mut().push(criterion);
    }
}
