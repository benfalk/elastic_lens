use super::*;
use std::borrow::Cow;

/// Aids in the construction of a script score
#[derive(Debug, Clone, Default)]
pub struct ScriptSortBuilder {
    source: Cow<'static, str>,
    params: Option<ScriptParams>,
    return_type: Option<ScriptReturnType>,
    direction: Option<SortDirection>,
}

impl ScriptSortBuilder {
    /// Start the construction of a script score
    pub fn new<S: Into<Cow<'static, str>>>(source: S) -> Self {
        Self {
            source: source.into(),
            params: None,
            return_type: None,
            direction: None,
        }
    }

    /// sort values ascending
    pub fn ascending(self) -> Self {
        Self {
            direction: Some(SortDirection::Ascending),
            ..self
        }
    }

    /// sort values decending
    pub fn descending(self) -> Self {
        Self {
            direction: Some(SortDirection::Descending),
            ..self
        }
    }

    /// script returns a number to sort
    pub fn returns_a_number(self) -> Self {
        Self {
            return_type: Some(ScriptReturnType::Number),
            ..self
        }
    }

    /// script returns a string to sort
    pub fn returns_a_string(self) -> Self {
        Self {
            return_type: Some(ScriptReturnType::String),
            ..self
        }
    }

    /// Attach params to the script score
    pub fn with_params<P: Into<ScriptParams>>(self, params: P) -> Self {
        Self {
            params: Some(params.into()),
            ..self
        }
    }

    /// creates a new [ScriptSort]
    pub fn build(self) -> ScriptSort {
        ScriptSort {
            return_type: self.return_type.unwrap_or_default(),
            params: self.params.unwrap_or_default(),
            direction: self.direction,
            source: self.source,
        }
    }
}

impl From<ScriptSortBuilder> for SortDirective {
    fn from(value: ScriptSortBuilder) -> Self {
        value.build().into()
    }
}
