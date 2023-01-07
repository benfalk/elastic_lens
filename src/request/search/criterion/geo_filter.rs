use super::*;
use serde::Serialize;

/// Matches geo_point and geo_shape values
/// within a given distance of a geopoint.
#[derive(Debug, Clone)]
pub struct GeoDistanceFilter {
    pub(super) field: Field,
    pub(super) distance: Distance,
    pub(super) point: GeoPoint,
}

impl GeoDistanceFilter {
    /// Starts the process of building a GeoDistanceFilter
    pub fn with_field<F: Into<Field>>(field: F) -> GeoDistanceFilterBuilder<1> {
        GeoDistanceFilterBuilder::for_field(field)
    }

    /// field filter is targeting
    pub fn field(&self) -> &Field {
        &self.field
    }

    /// the distance from the point to select from field
    pub fn distance(&self) -> Distance {
        self.distance
    }

    /// the focal point from which the radius filter is done
    pub fn point(&self) -> GeoPoint {
        self.point
    }
}

impl From<GeoDistanceFilter> for Criterion {
    fn from(value: GeoDistanceFilter) -> Self {
        Self::GeoDistance(value)
    }
}

impl CriterionData for GeoDistanceFilter {}

#[derive(Debug, Clone)]
#[must_use = "must be consumed to build a GeoDistanceFilter"]
#[doc(hidden)]
pub struct GeoDistanceFilterBuilder<const L: u8> {
    field: Option<Field>,
    unit: Option<usize>,
    distance: Option<Distance>,
    point: Option<GeoPoint>,
}

impl GeoDistanceFilterBuilder<1> {
    pub fn for_field<F: Into<Field>>(field: F) -> GeoDistanceFilterBuilder<1> {
        GeoDistanceFilterBuilder {
            field: Some(field.into()),
            unit: None,
            distance: None,
            point: None,
        }
    }
}

impl GeoDistanceFilterBuilder<1> {
    /// the number of units away
    pub fn within(self, unit: usize) -> GeoDistanceFilterBuilder<2> {
        GeoDistanceFilterBuilder {
            unit: Some(unit),
            field: self.field,
            distance: self.distance,
            point: self.point,
        }
    }
}

impl GeoDistanceFilterBuilder<2> {
    /// the units in miles
    pub fn miles(self) -> GeoDistanceFilterBuilder<3> {
        GeoDistanceFilterBuilder {
            unit: None,
            field: self.field,
            distance: Some(Distance::Miles(self.unit.unwrap())),
            point: self.point,
        }
    }

    /// the units in kilometers
    pub fn kilomenters(self) -> GeoDistanceFilterBuilder<3> {
        GeoDistanceFilterBuilder {
            unit: None,
            field: self.field,
            distance: Some(Distance::Kilometers(self.unit.unwrap())),
            point: self.point,
        }
    }
}

impl GeoDistanceFilterBuilder<3> {
    /// target a point to measure the distance from
    /// for the field.  This finishes the contruction
    /// of and it returns the `GeoDistanceFilter`
    pub fn of<P: IntoGeoPoint>(self, geopoint: P) -> GeoDistanceFilter {
        GeoDistanceFilter {
            field: self.field.unwrap(),
            distance: self.distance.unwrap(),
            point: geopoint.into_geo_point(),
        }
    }
}

impl Serialize for GeoDistanceFilter {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;

        struct DistanceString(Distance);

        impl Serialize for DistanceString {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                use std::fmt::Write;
                let mut string = arraystring::SmallString::new();
                match self.0 {
                    Distance::Miles(amount) => write!(&mut string, "{}mi", amount)
                        .or_else(|_| Err(serde::ser::Error::custom("Cannot write miles")))?,
                    Distance::Kilometers(amount) => write!(&mut string, "{}km", amount)
                        .or_else(|_| Err(serde::ser::Error::custom("Cannot write kilomenters")))?,
                };
                serializer.serialize_str(&string)
            }
        }

        struct DistanceFieldLocation<'a> {
            distance: DistanceString,
            field: &'a Field,
            location: GeoPoint,
        }

        impl<'a> Serialize for DistanceFieldLocation<'a> {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                let mut map = serializer.serialize_map(Some(2))?;
                map.serialize_entry("distance", &self.distance)?;
                map.serialize_entry(self.field, &self.location)?;
                map.end()
            }
        }

        let dfl = DistanceFieldLocation {
            distance: DistanceString(self.distance),
            field: &self.field,
            location: self.point,
        };
        let mut map = serializer.serialize_map(Some(1))?;
        map.serialize_entry("geo_distance", &dfl)?;
        map.end()
    }
}
