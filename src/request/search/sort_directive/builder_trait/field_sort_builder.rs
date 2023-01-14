use super::*;

/// The first step towards building a sort where a document
/// field is the target
#[derive(Debug)]
pub struct FieldSortBuilder {
    pub(super) field: Field,
}

impl FieldSortBuilder {
    /// Creates a geo-distance sort. There are a number of options
    /// you can add to this sort, but when you end the chain it
    /// will build with whatever options you take.
    pub fn by_distance_from<P: IntoGeoPoint>(self, location: P) -> GeoDistanceSortBuilder {
        GeoDistanceSortBuilder {
            field: self.field,
            location: location.into_geo_point(),
            order: None,
            ignore_unmapped: None,
            calc_formula: None,
        }
    }

    /// sort field in decending order
    pub fn descending(self) -> FieldSortBuilderOptions {
        FieldSortBuilderOptions {
            field: self.field,
            direction: SortDirection::Descending,
            missing_value: None,
        }
    }

    /// sort field in ascending order
    pub fn ascending(self) -> FieldSortBuilderOptions {
        FieldSortBuilderOptions {
            field: self.field,
            direction: SortDirection::Ascending,
            missing_value: None,
        }
    }
}

impl From<FieldSortBuilder> for SortDirective {
    fn from(value: FieldSortBuilder) -> Self {
        Self::Field(SortField {
            field: value.field,
            direction: None,
            missing_value: None,
        })
    }
}
