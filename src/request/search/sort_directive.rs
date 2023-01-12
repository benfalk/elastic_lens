use super::*;
use serde::Serialize;

mod builder_trait;
mod direction;
mod field_params;
mod geo_params;
mod missing_value;

pub use builder_trait::*;
pub use direction::*;
pub use field_params::*;
pub use geo_params::*;
pub use missing_value::*;

/// Describes a way to sort documents from a search
#[derive(Debug, Clone)]
pub enum SortDirective {
    /// Sort by Field
    Field(SortField),

    /// Sort from GeoPoint
    GeoDistance(SortGeo),
}

impl Serialize for SortDirective {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Self::Field(params) => params.serialize(serializer),
            Self::GeoDistance(params) => params.serialize(serializer),
        }
    }
}
