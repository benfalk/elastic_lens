use super::*;
use serde::{ser::SerializeMap, Serialize};

/// Information needed to sort results from
/// a geo-point location
#[derive(Debug, Clone)]
pub struct SortGeo {
    pub(super) field: Field,
    pub(super) location: GeoPoint,
    pub(super) order: Option<SortDirection>,
    pub(super) ignore_unmapped: Option<bool>,
    pub(super) calc_formula: Option<CalculationFormula>,
}

impl SortGeo {
    /// Starts construction on a geo-distance sort
    pub fn field_and_location<F, P>(field: F, location: P) -> SortGeoBuilder
    where
        F: Into<Field>,
        P: IntoGeoPoint,
    {
        SortGeoBuilder {
            field: field.into(),
            location: location.into_geo_point(),
            order: None,
            ignore_unmapped: None,
            calc_formula: None,
        }
    }
}

impl From<SortGeo> for SortDirective {
    fn from(value: SortGeo) -> Self {
        Self::GeoDistance(value)
    }
}

/// Aids in construcution of a geo-distance sort
#[derive(Debug, Clone)]
#[must_use = "must call build() to creat a geo-distance sort"]
pub struct SortGeoBuilder {
    field: Field,
    location: GeoPoint,
    order: Option<SortDirection>,
    ignore_unmapped: Option<bool>,
    calc_formula: Option<CalculationFormula>,
}

impl SortGeoBuilder {
    /// finish construction of a geo-distance sort
    pub fn build(self) -> SortGeo {
        SortGeo {
            field: self.field,
            location: self.location,
            order: self.order,
            ignore_unmapped: self.ignore_unmapped,
            calc_formula: self.calc_formula,
        }
    }

    /// Sort ascending
    pub fn in_ascending_order(self) -> Self {
        Self {
            order: Some(SortDirection::Ascending),
            ..self
        }
    }

    /// Sort descending
    pub fn in_descending_order(self) -> Self {
        Self {
            order: Some(SortDirection::Descending),
            ..self
        }
    }

    /// Arc is normally the default, and most accurate
    pub fn using_the_arc_formula(self) -> Self {
        Self {
            calc_formula: Some(CalculationFormula::Arc),
            ..self
        }
    }

    /// Plane is less accurate, especially over longer
    /// distances and near the poles; however, it is much
    /// faster
    pub fn using_the_plane_formula(self) -> Self {
        Self {
            calc_formula: Some(CalculationFormula::Plane),
            ..self
        }
    }

    /// By default Elasticsearch will error on a document
    /// if it's missing the field being used; setting this
    /// will instead allow the search to work and the documents
    /// missing the distance will be considered "Infinity"
    pub fn ignore_unmapped_documents(self) -> Self {
        Self {
            ignore_unmapped: Some(true),
            ..self
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub(super) enum CalculationFormula {
    Arc,
    Plane,
}

impl Serialize for CalculationFormula {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Self::Arc => "arc".serialize(serializer),
            Self::Plane => "plane".serialize(serializer),
        }
    }
}

impl Serialize for SortGeo {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        struct Params<'a> {
            field: &'a Field,
            location: &'a GeoPoint,
            order: &'a Option<SortDirection>,
            ignore_unmapped: &'a Option<bool>,
            calc_formula: &'a Option<CalculationFormula>,
        }

        impl<'a> Params<'a> {
            fn map_size(&self) -> usize {
                [
                    self.order.is_some(),
                    self.ignore_unmapped.is_some(),
                    self.calc_formula.is_some(),
                ]
                .iter()
                .filter(|something| **something)
                .count()
                    + 1
            }

            fn borrow(sort_geo: &'a SortGeo) -> Self {
                Self {
                    field: &sort_geo.field,
                    location: &sort_geo.location,
                    order: &sort_geo.order,
                    ignore_unmapped: &sort_geo.ignore_unmapped,
                    calc_formula: &sort_geo.calc_formula,
                }
            }
        }

        impl<'a> Serialize for Params<'a> {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                let mut map = serializer.serialize_map(Some(self.map_size()))?;

                map.serialize_entry(self.field, self.location)?;

                if let Some(order) = self.order {
                    map.serialize_entry("order", order)?;
                }

                if let Some(formula) = self.calc_formula {
                    map.serialize_entry("distance_type", formula)?;
                }

                if let Some(ignore) = self.ignore_unmapped {
                    map.serialize_entry("ignore_unmapped", ignore)?;
                }

                map.end()
            }
        }

        let params = Params::borrow(self);
        let mut map = serializer.serialize_map(Some(1))?;
        map.serialize_entry("_geo_distance", &params)?;
        map.end()
    }
}
