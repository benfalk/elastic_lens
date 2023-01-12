use super::*;

/// Aids in building a geo-distance sort for `FieldSortBuilder`
#[derive(Debug)]
pub struct GeoDistanceSortBuilder<'a> {
    pub(super) field: Field,
    pub(super) location: GeoPoint,
    pub(super) sorts: &'a mut Vec<SortDirective>,
    pub(super) order: Option<SortDirection>,
    pub(super) ignore_unmapped: Option<bool>,
    pub(super) calc_formula: Option<CalculationFormula>,
}

impl GeoDistanceSortBuilder<'_> {
    /// Sort ascending
    pub fn in_ascending_order(&mut self) -> &mut Self {
        self.order = Some(SortDirection::Ascending);
        self
    }

    /// Sort descending
    pub fn in_descending_order(&mut self) -> &mut Self {
        self.order = Some(SortDirection::Descending);
        self
    }

    /// Arc is normally the default, and most accurate
    pub fn using_the_arc_formula(&mut self) -> &mut Self {
        self.calc_formula = Some(CalculationFormula::Arc);
        self
    }

    /// Plane is less accurate, especially over longer
    /// distances and near the poles; however, it is much
    /// faster
    pub fn using_the_plane_formula(&mut self) -> &mut Self {
        self.calc_formula = Some(CalculationFormula::Plane);
        self
    }

    /// By default Elasticsearch will error on a document
    /// if it's missing the field being used; setting this
    /// will instead allow the search to work and the documents
    /// missing the distance will be considered "Infinity"
    pub fn ignore_unmapped_documents(&mut self) -> &mut Self {
        self.ignore_unmapped = Some(true);
        self
    }
}

impl Drop for GeoDistanceSortBuilder<'_> {
    fn drop(&mut self) {
        let mut field = "".into();
        let mut location = GeoPoint::default();
        std::mem::swap(&mut field, &mut self.field);
        std::mem::swap(&mut location, &mut self.location);

        let sort = SortGeo {
            field,
            location,
            order: self.order.take(),
            ignore_unmapped: self.ignore_unmapped.take(),
            calc_formula: self.calc_formula.take(),
        };

        self.sorts.push(sort.into());
    }
}
