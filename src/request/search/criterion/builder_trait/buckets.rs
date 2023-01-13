use super::*;

/// ZST trait tag to decide where criteria are placed for a builder
pub trait BucketPlacer {
    /// place the criterion where it makes sense to
    fn push<C, B>(builder: &mut B, criterion: C, tag: Tag)
    where
        C: Into<Criterion>,
        B: CriteriaBuilder;
}

/// ZST struct which will place criteria into the positive
/// bucket.  This also helps to switch to a "negative" bucket
/// when in positive mode and avoid nested "nots".
#[derive(Debug, Copy, Clone)]
pub struct NormalBucket {}

impl BucketPlacer for NormalBucket {
    fn push<C, B>(builder: &mut B, criterion: C, tag: Tag)
    where
        C: Into<Criterion>,
        B: CriteriaBuilder,
    {
        match tag {
            Tag::Positive => {
                builder.positive_criteria_mut().push(criterion.into());
            }
            Tag::Negative => {
                builder.negative_criteria_mut().push(criterion.into());
            }
        }
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
    fn push<C, B>(builder: &mut B, criterion: C, tag: Tag)
    where
        C: Into<Criterion>,
        B: CriteriaBuilder,
    {
        match tag {
            Tag::Positive => {
                builder.positive_criteria_mut().push(criterion.into());
            }
            Tag::Negative => {
                let not_all = NotAll::single(criterion);
                builder.positive_criteria_mut().push(not_all.into());
            }
        }
    }
}
