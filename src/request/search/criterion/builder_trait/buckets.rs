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

/// Tag trait to notifiy that we are in an "if_any_match"
/// block and that certain functionality is available such
/// as the `if_all_match` function.
pub trait OrBucketPlacer: BucketPlacer {}

/// ZST struct which places criteria into a positive
/// bucket with the expectation that negatives will
/// be forced also into the positive bucket but jailed
/// separately into their own single NOT node.
#[derive(Debug, Copy, Clone)]
pub struct GroupedOrBucket {}

impl BucketPlacer for GroupedOrBucket {
    fn push<C, B>(builder: &mut B, criterion: C)
    where
        C: Into<Criterion>,
        B: CriteriaBuilder,
    {
        builder.positive_criteria_mut().push(criterion.into());
    }
}

impl OrBucketPlacer for GroupedOrBucket {}

/// This is the tag that lets the builder trait know
/// it has transitioned from the `GroupedOrBucket`
/// to a negative condition and that it needs to be
/// jailed by itself.
#[derive(Debug, Copy, Clone)]
pub struct NegativeGroupedOrBucket {}

impl BucketPlacer for NegativeGroupedOrBucket {
    fn push<C, B>(builder: &mut B, criterion: C)
    where
        C: Into<Criterion>,
        B: CriteriaBuilder,
    {
        let not_all = NotAll::single(criterion);
        builder.positive_criteria_mut().push(not_all.into());
    }
}
