use super::*;
use std::borrow::Cow;

/// Allows for dynamic script filtering of documents.  This is a
/// powerful but expensive way to filter, so use with care.
#[derive(Debug, Clone)]
pub struct ScriptFilter {
    script: Cow<'static, str>,
    params: ScriptParams,
}

/// Aids in the construction of a script filter
#[derive(Debug, Clone, Default)]
pub struct ScriptFilterBuilder {
    script: Cow<'static, str>,
    params: ScriptParams,
}

impl ScriptFilterBuilder {
    /// Start the construction of a script filter with it's source
    pub fn new<S: Into<Cow<'static, str>>>(script: S) -> Self {
        Self {
            script: script.into(),
            params: ScriptParams::default(),
        }
    }

    /// Attach params to the script filter
    pub fn with_params<P: Into<ScriptParams>>(mut self, params: P) -> Self {
        self.params = params.into();
        self
    }

    /// Builds the script filter
    pub fn build(self) -> ScriptFilter {
        ScriptFilter {
            script: self.script,
            params: self.params,
        }
    }
}

impl CriterionData for ScriptFilter {}

impl From<ScriptFilter> for Criterion {
    fn from(value: ScriptFilter) -> Self {
        Self::ScriptFilter(value)
    }
}

impl From<ScriptFilterBuilder> for Criterion {
    fn from(value: ScriptFilterBuilder) -> Self {
        value.build().into()
    }
}

impl Serialize for ScriptFilter {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        #[derive(Serialize)]
        struct ScriptData<'a> {
            source: &'a Cow<'static, str>,

            #[serde(skip_serializing_if = "ScriptParams::is_empty")]
            params: &'a ScriptParams,
        }

        #[derive(Serialize)]
        struct ScriptContainer<'a> {
            script: ScriptData<'a>,
        }

        #[derive(Serialize)]
        struct TagContainer<'a> {
            script: ScriptContainer<'a>,
        }

        TagContainer {
            script: ScriptContainer {
                script: ScriptData {
                    source: &self.script,
                    params: &self.params,
                },
            },
        }
        .serialize(serializer)
    }
}
