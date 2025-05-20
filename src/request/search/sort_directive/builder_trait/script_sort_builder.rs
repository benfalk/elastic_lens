use super::*;
use std::borrow::Cow;

/// Aids in the construction of a script score
#[derive(Debug, Clone)]
pub struct ScriptScoreBuilder {
    script: Cow<'static, str>,
    params: Option<ScriptParams>,
    direction: Option<SortDirection>,
}

impl ScriptScoreBuilder {
    /// Start the construction of a script score
    pub fn new<S: Into<Cow<'static, str>>>(script: S) -> Self {
        Self {
            script: script.into(),
            params: None,
            direction: None,
        }
    }

    /// Attach params to the script score
    pub fn with_params<P: Into<ScriptParams>>(self, params: P) -> Self {
        Self {
            params: Some(params.into()),
            ..self
        }
    }

    /// Sort the script ascending
    pub fn sort_ascending(self) -> Self {
        Self {
            direction: Some(SortDirection::Ascending),
            ..self
        }
    }

    /// Sort the script descending
    pub fn sort_decending(self) -> Self {
        Self {
            direction: Some(SortDirection::Descending),
            ..self
        }
    }

    /// Create a new script-score
    pub fn build(self) -> ScriptScoreData {
        ScriptScoreData {
            script: self.script,
            params: self.params.unwrap_or_default(),
            direction: self.direction,
        }
    }
}

impl From<ScriptScoreBuilder> for SortDirective {
    fn from(value: ScriptScoreBuilder) -> Self {
        value.build().into()
    }
}
