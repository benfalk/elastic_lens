use super::*;
use serde::Serialize;
use std::borrow::Cow;
use std::collections::HashMap;

#[derive(Debug, Clone)]
#[doc(hidden)]
pub enum ScriptValue {
    Scalar(ScalarValue),
    Array(Vec<ScalarValue>),
}

impl Serialize for ScriptValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Self::Scalar(value) => value.serialize(serializer),
            Self::Array(vec) => vec.serialize(serializer),
        }
    }
}

/// Data to be used by a script sort
#[derive(Debug, Clone, Default)]
pub struct ScriptParams {
    data: HashMap<Cow<'static, str>, ScriptValue>,
}

impl<I, K, S> From<I> for ScriptParams
where
    S: Into<ScalarValue>,
    K: Into<Cow<'static, str>>,
    I: IntoIterator<Item = (K, S)>,
{
    fn from(value: I) -> Self {
        let mut params = Self::default();
        for (key, value) in value.into_iter() {
            params
                .data
                .insert(key.into(), ScriptValue::Scalar(value.into()));
        }
        params
    }
}

impl ScriptParams {
    /// Use when you know how many params ahead of time you'll need
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            data: HashMap::with_capacity(capacity),
        }
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Add a single value
    pub fn insert_scalar<K, V>(&mut self, key: K, value: V)
    where
        K: Into<Cow<'static, str>>,
        V: Into<ScalarValue>,
    {
        let cow_key = key.into();
        let scalar = ScriptValue::Scalar(value.into());
        self.data.insert(cow_key, scalar);
    }

    /// Add an array of values
    pub fn insert_array<K, V, S>(&mut self, key: K, value: V)
    where
        K: Into<Cow<'static, str>>,
        S: Into<ScalarValue>,
        V: IntoIterator<Item = S>,
    {
        let cow_key = key.into();
        let vec = value.into_iter().map(Into::into).collect();
        let array = ScriptValue::Array(vec);
        self.data.insert(cow_key, array);
    }
}

impl Serialize for ScriptParams {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.data.serialize(serializer)
    }
}
