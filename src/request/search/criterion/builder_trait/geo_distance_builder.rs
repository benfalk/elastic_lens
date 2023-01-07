use super::*;
use std::marker::PhantomData;

#[derive(Debug)]
#[must_use = "must pick one of `miles` or `kilometers` building"]
#[doc(hidden)]
pub struct UnitPicker<'a, T: BucketPlacer, B: CriteriaBuilder> {
    pub(super) marker: PhantomData<T>,
    pub(super) builder: &'a mut B,
    pub(super) field: Field,
    pub(super) amount: usize,
}

impl<'a, T: BucketPlacer, B: CriteriaBuilder> UnitPicker<'a, T, B> {
    /// Set the units in miles
    pub fn miles(self) -> PointPicker<'a, T, B> {
        let distance = Distance::Miles(self.amount);

        PointPicker {
            marker: PhantomData,
            builder: self.builder,
            field: self.field,
            distance,
        }
    }

    /// Set the units in kilometers
    pub fn kilomenters(self) -> PointPicker<'a, T, B> {
        let distance = Distance::Kilometers(self.amount);

        PointPicker {
            marker: PhantomData,
            builder: self.builder,
            field: self.field,
            distance,
        }
    }
}

#[derive(Debug)]
#[must_use = "must use 'of' to finish building"]
#[doc(hidden)]
pub struct PointPicker<'a, T: BucketPlacer, B: CriteriaBuilder> {
    marker: PhantomData<T>,
    builder: &'a mut B,
    field: Field,
    distance: Distance,
}

impl<'a, T: BucketPlacer, B: CriteriaBuilder> PointPicker<'a, T, B> {
    /// Pick a location to measure from
    pub fn of<P: IntoGeoPoint>(self, point: P) {
        let filter = GeoDistanceFilter {
            field: self.field,
            distance: self.distance,
            point: point.into_geo_point(),
        };

        T::push(self.builder, filter);
    }
}
