use super::*;

/// The first step towards building a sort where a document
/// field is the target
#[derive(Debug)]
#[must_use = "A selection must be made to build a sort"]
pub struct FieldSortBuilder<'a> {
    pub(super) field: Field,
    pub(super) sorts: &'a mut Vec<SortDirective>,
}

impl<'a> FieldSortBuilder<'a> {
    /// Creates a geo-distance sort. There are a number of options
    /// you can add to this sort, but when you end the chain it
    /// will build with whatever options you take.
    pub fn by_distance_from<P: IntoGeoPoint>(self, location: P) -> GeoDistanceSortBuilder<'a> {
        GeoDistanceSortBuilder {
            field: self.field,
            location: location.into_geo_point(),
            sorts: self.sorts,
            order: None,
            ignore_unmapped: None,
            calc_formula: None,
        }
    }

    /// sort field in decending order
    pub fn descending(self) -> FieldSortBuilderOptions<'a> {
        FieldSortBuilderOptions {
            field: self.field,
            sorts: self.sorts,
            direction: SortDirection::Descending,
            missing_value: None,
        }
    }

    /// sort field in ascending order
    pub fn ascending(self) -> FieldSortBuilderOptions<'a> {
        FieldSortBuilderOptions {
            field: self.field,
            sorts: self.sorts,
            direction: SortDirection::Ascending,
            missing_value: None,
        }
    }
}
