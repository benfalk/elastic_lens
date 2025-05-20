use super::*;
use serde::Serialize;
use std::borrow::Cow;

mod params;
pub use params::*;

/// Data used to build a `script_score` for an Elasticsearch
/// request.  Currently this is not a full representation of
/// what is used, but instead the most common.
#[derive(Debug, Clone, Default, Serialize)]
pub struct ScriptScoreData {
    #[serde(rename = "source")]
    pub(super) script: Cow<'static, str>,

    #[serde(skip_serializing_if = "ScriptParams::is_empty")]
    pub(super) params: ScriptParams,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) direction: Option<SortDirection>,
}

impl ScriptScoreData {
    /// Creates a new script sort w/o any params
    pub fn new<S: Into<Cow<'static, str>>>(script: S) -> Self {
        Self {
            script: script.into(),
            params: ScriptParams::default(),
            direction: None,
        }
    }

    /// Creates a new script sort with params
    pub fn new_with_params<S>(script: S, params: ScriptParams) -> Self
    where
        S: Into<Cow<'static, str>>,
    {
        Self {
            params,
            script: script.into(),
            direction: None,
        }
    }
}

impl From<ScriptScoreData> for SortDirective {
    fn from(value: ScriptScoreData) -> Self {
        Self::ScriptScore(value)
    }
}
