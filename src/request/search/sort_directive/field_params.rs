use super::*;
use serde::Serialize;

/// How to sort documents by field
#[derive(Debug, Clone)]
pub struct SortField {
    pub(super) field: Field,
    pub(super) direction: Option<SortDirection>,
    pub(super) missing_value: Option<MissingValue>,
}

impl SortField {
    /// Starts the construction of a new field sort by
    /// providing a builder that can set optional parameters
    /// before building
    pub fn field<F: Into<Field>>(field: F) -> SortFieldBuilder {
        SortFieldBuilder {
            field: field.into(),
            direction: None,
            missing_value: None,
        }
    }
}

impl From<SortField> for SortDirective {
    fn from(value: SortField) -> Self {
        Self::Field(value)
    }
}

/// Builder used to aid in construction of `SortFieldParams`
#[derive(Debug, Clone)]
#[must_use = "must be consumed to build a SortFieldParams"]
pub struct SortFieldBuilder {
    field: Field,
    direction: Option<SortDirection>,
    missing_value: Option<MissingValue>,
}

impl SortFieldBuilder {
    /// create a new field sort
    pub fn build(self) -> SortField {
        SortField {
            field: self.field,
            direction: self.direction,
            missing_value: self.missing_value,
        }
    }

    /// Sort ascending
    pub fn in_ascending_order(self) -> Self {
        Self {
            direction: Some(SortDirection::Ascending),
            ..self
        }
    }

    /// Sort descending
    pub fn in_descending_order(self) -> Self {
        Self {
            direction: Some(SortDirection::Descending),
            ..self
        }
    }

    /// If the field is missing a value, use this in those cases
    pub fn where_missing_use<V: Into<ScalarValue>>(self, value: V) -> Self {
        Self {
            missing_value: Some(MissingValue::Custom(value.into())),
            ..self
        }
    }

    /// Documents without a value are sorted last
    pub fn with_missing_values_last(self) -> Self {
        Self {
            missing_value: Some(MissingValue::Last),
            ..self
        }
    }

    /// Documents without a value are sorted first
    pub fn with_missing_values_first(self) -> Self {
        Self {
            missing_value: Some(MissingValue::First),
            ..self
        }
    }
}

impl Serialize for SortField {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;

        #[derive(Serialize)]
        struct Params<'a> {
            #[serde(skip_serializing_if = "Option::is_none")]
            order: &'a Option<SortDirection>,
            #[serde(skip_serializing_if = "Option::is_none")]
            missing: &'a Option<MissingValue>,
        }

        if self.direction.is_none() && self.missing_value.is_none() {
            return self.field.serialize(serializer);
        }

        let mut map = serializer.serialize_map(Some(1))?;

        if let (Some(direction), None) = (&self.direction, &self.missing_value) {
            map.serialize_entry(&self.field, direction)?;
            return map.end();
        }

        let params = Params {
            order: &self.direction,
            missing: &self.missing_value,
        };

        map.serialize_entry(&self.field, &params)?;
        map.end()
    }
}
