use super::*;

/// ZST trait tag to decide where criteria are placed for a builder
pub trait BucketPlacer {
    /// place the criterion where it makes sense to
    fn push<C, B>(builder: &mut B, criterion: C)
    where
        C: Into<Criterion>,
        B: CriteriaBuilder;
}

/// ZST struct which will place criteria into the positive
/// bucket.  This also helps to switch to a "negative" bucket
/// when in positive mode and avoid nested "nots".
#[derive(Debug, Copy, Clone)]
pub struct PositiveBucket {}

impl BucketPlacer for PositiveBucket {
    fn push<C, B>(builder: &mut B, criterion: C)
    where
        C: Into<Criterion>,
        B: CriteriaBuilder,
    {
        builder.positive_criteria_mut().push(criterion.into());
    }
}

/// ZST struct which places criteria into the negative bucket.
#[derive(Debug, Copy, Clone)]
pub struct NegativeBucket {}

impl BucketPlacer for NegativeBucket {
    fn push<C, B>(builder: &mut B, criterion: C)
    where
        C: Into<Criterion>,
        B: CriteriaBuilder,
    {
        builder.negative_criteria_mut().push(criterion.into());
    }
}
