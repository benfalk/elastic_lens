use super::*;
use serde::Serialize;

/// Selects if the field has any values
#[derive(Debug, Clone)]
pub struct ExistsFilter {
    field: Field,
}

impl ExistsFilter {
    /// Creates an ExistsFilter
    pub fn new<F: Into<Field>>(field: F) -> Self {
        Self {
            field: field.into(),
        }
    }
}

impl CriterionData for ExistsFilter {}

impl From<ExistsFilter> for Criterion {
    fn from(value: ExistsFilter) -> Self {
        Self::Exists(value)
    }
}

impl Serialize for ExistsFilter {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;

        #[derive(Serialize)]
        struct Params<'a> {
            field: &'a Field,
        }

        let params = Params { field: &self.field };
        let mut map = serializer.serialize_map(Some(1))?;
        map.serialize_entry("exists", &params)?;
        map.end()
    }
}
