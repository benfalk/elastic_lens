use serde::Serialize;
use std::borrow::Cow;

/// Elasticsearch Search Field
///
/// This encasulates the mapping/field on the Elasticsearch index
/// that you want to target for a search or other potential action.
///
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Field {
    name: Cow<'static, str>,
}

impl From<&'static str> for Field {
    fn from(value: &'static str) -> Self {
        Self {
            name: Cow::Borrowed(value),
        }
    }
}

impl From<String> for Field {
    fn from(value: String) -> Self {
        Self {
            name: Cow::Owned(value),
        }
    }
}

impl Field {
    /// Static Field
    ///
    /// Const function which allows to declare const fields in your
    /// project without needing something like `lazy_static`.
    pub const fn static_field(name: &'static str) -> Self {
        Self {
            name: Cow::Borrowed(name),
        }
    }
}

impl Serialize for Field {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.name)
    }
}

/// Create Public Constant Field
///
/// Convenience macro to make it more easy to define
/// constant fields in your project.
#[macro_export]
macro_rules! pub_field {
    ($name:ident, $value:literal) => {
        /// Field for $value
        pub const $name: $crate::request::search::Field =
            $crate::request::search::Field::static_field($value);
    };
}
