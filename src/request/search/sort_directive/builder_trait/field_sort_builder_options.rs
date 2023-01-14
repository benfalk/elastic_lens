use super::*;

/// Aids in finishing a field sort for a search
#[derive(Debug)]
pub struct FieldSortBuilderOptions {
    pub(super) field: Field,
    pub(super) direction: SortDirection,
    pub(super) missing_value: Option<MissingValue>,
}

impl FieldSortBuilderOptions {
    /// If the field is missing a value, use this in those cases
    pub fn where_missing_use<V: Into<ScalarValue>>(mut self, value: V) -> Self {
        self.missing_value = Some(MissingValue::Custom(value.into()));
        self
    }

    /// Documents without a value are sorted last
    pub fn with_missing_values_last(mut self) -> Self {
        self.missing_value = Some(MissingValue::Last);
        self
    }

    /// Documents without a value are sorted first
    pub fn with_missing_values_first(mut self) -> Self {
        self.missing_value = Some(MissingValue::First);
        self
    }
}

impl From<FieldSortBuilderOptions> for SortDirective {
    fn from(value: FieldSortBuilderOptions) -> Self {
        Self::Field(SortField {
            field: value.field,
            direction: Some(value.direction),
            missing_value: value.missing_value,
        })
    }
}
