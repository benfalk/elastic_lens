use super::*;

/// Aids in finishing a field sort for a search
#[derive(Debug)]
pub struct FieldSortBuilderOptions<'a> {
    pub(super) field: Field,
    pub(super) sorts: &'a mut Vec<SortDirective>,
    pub(super) direction: SortDirection,
    pub(super) missing_value: Option<MissingValue>,
}

impl<'a> FieldSortBuilderOptions<'a> {
    /// If the field is missing a value, use this in those cases
    pub fn where_missing_use<V: Into<ScalarValue>>(mut self, value: V) {
        self.missing_value = Some(MissingValue::Custom(value.into()));
    }

    /// Documents without a value are sorted last
    pub fn with_missing_values_last(mut self) {
        self.missing_value = Some(MissingValue::Last);
    }

    /// Documents without a value are sorted first
    pub fn with_missing_values_first(mut self) {
        self.missing_value = Some(MissingValue::First);
    }
}

impl<'a> Drop for FieldSortBuilderOptions<'a> {
    fn drop(&mut self) {
        let mut field = "".into();
        let mut direction = SortDirection::Ascending;
        std::mem::swap(&mut field, &mut self.field);
        std::mem::swap(&mut direction, &mut self.direction);

        let sort = SortField {
            field,
            direction: Some(direction),
            missing_value: self.missing_value.take(),
        };

        self.sorts.push(sort.into());
    }
}
