use super::*;
use serde::Serialize;

mod builder_trait;
mod direction;
mod field_params;
mod geo_params;
mod missing_value;
mod script_params;
mod script_score;
mod script_sort;

pub use builder_trait::*;
pub use direction::*;
pub use field_params::*;
pub use geo_params::*;
pub use missing_value::*;
pub use script_params::*;
pub use script_score::*;
pub use script_sort::*;

/// Describes a way to sort documents from a search
#[derive(Debug, Clone)]
pub enum SortDirective {
    /// Sort by Field
    Field(SortField),

    /// Sort from GeoPoint
    GeoDistance(SortGeo),

    /// When sorting with a script score
    ScriptScore(ScriptScoreData),

    /// When sorting with a script
    ScriptSort(ScriptSort),
}

impl SortDirective {
    /// Determines if the sort is a script-score sort.
    /// This kind of sort is presented much differently
    /// to Elasticsearch so the distinction needs to be
    /// known.
    pub(crate) fn is_script_score(&self) -> bool {
        matches!(self, Self::ScriptScore(_))
    }
}

impl Serialize for SortDirective {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Self::Field(params) => params.serialize(serializer),
            Self::GeoDistance(params) => params.serialize(serializer),
            Self::ScriptScore(params) => params.serialize(serializer),
            Self::ScriptSort(params) => params.serialize(serializer),
        }
    }
}
