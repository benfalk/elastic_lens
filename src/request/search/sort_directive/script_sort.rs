use super::*;
use serde::ser::Serialize;
use std::borrow::Cow;

/// Describes what primitive the script returns
#[derive(Debug, Clone, Copy, Default)]
pub enum ScriptReturnType {
    /// script returns a number to sort
    #[default]
    Number,

    /// script returns a string to sort
    String,
}

/// Sort with a script
#[derive(Debug, Clone)]
pub struct ScriptSort {
    pub(super) return_type: ScriptReturnType,
    pub(super) params: ScriptParams,
    pub(super) direction: Option<SortDirection>,
    pub(super) source: Cow<'static, str>,
}

// Serialize Impls

impl Serialize for ScriptReturnType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Self::Number => "number".serialize(serializer),
            Self::String => "string".serialize(serializer),
        }
    }
}

impl Serialize for ScriptSort {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;

        #[derive(Serialize)]
        struct TopLevel<'a> {
            #[serde(rename = "type")]
            return_type: &'a ScriptReturnType,
            #[serde(skip_serializing_if = "Option::is_none")]
            order: Option<&'a SortDirection>,
            script: ScriptDetails<'a>,
        }

        impl<'a> TopLevel<'a> {
            fn new(script: &'a ScriptSort) -> Self {
                Self {
                    return_type: &script.return_type,
                    order: script.direction.as_ref(),
                    script: ScriptDetails {
                        source: &script.source,
                        params: &script.params,
                    },
                }
            }
        }

        #[derive(Serialize)]
        struct ScriptDetails<'a> {
            source: &'a Cow<'static, str>,
            #[serde(skip_serializing_if = "ScriptParams::is_empty")]
            params: &'a ScriptParams,
        }

        let mut map = serializer.serialize_map(Some(1))?;
        map.serialize_entry("_script", &TopLevel::new(self))?;
        map.end()
    }
}

impl From<ScriptSort> for SortDirective {
    fn from(value: ScriptSort) -> Self {
        Self::ScriptSort(value)
    }
}
