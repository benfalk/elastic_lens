use super::*;
use serde::de::{self, Deserialize, Deserializer, MapAccess, Visitor};
use std::collections::BTreeMap;

/// Errors that can happen when trying to work with aggregations
#[derive(Debug, thiserror::Error, Clone)]
pub enum AggAccessError {
    /// Variant when you attempt to borrow an underlying aggregation
    /// by the wrong type from an `AggResult`
    #[error("incorrect access type, expected {expected}, but got {got}")]
    WrongType {
        /// string represention of what type was expected
        expected: &'static str,

        /// string represention of what type was actually present
        got: &'static str,
    },

    /// Returned when the key attempting to access doesn't exist
    #[error("aggregation not found under key: {0}")]
    AggNotFound(String),
}

/// Aggregation Results from ElasticSearch
///
/// This is the access structure to retrieve your
/// aggregation results by name from your request.
#[derive(Debug, Clone, Default)]
pub struct AggResultCollection {
    data: BTreeMap<String, AggResult>,
}

impl AggResultCollection {
    /// Retrieves the aggregation by it's name and
    /// "casts" it down to the correct underlying type.
    ///
    /// This will fail if either the key is not found
    /// or it incorrectly casts down to the wrong type.
    ///
    /// Note this doesn't do any actual downcast but instead
    /// uses enum matching.
    pub fn get<T: AggResultData>(&self, name: &str) -> Result<&T, AggAccessError> {
        let agg = self
            .data
            .get(name)
            .ok_or_else(|| AggAccessError::AggNotFound(name.to_owned()))?;

        T::borrow_agg_result(agg)
    }

    /// Retrieves the aggregation by it's name and
    /// "casts" it down to the correct underlying type.
    ///
    /// This will fail if either the key is not found
    /// or it incorrectly casts down to the wrong type.
    ///
    /// Note this doesn't do any actual downcast but instead
    /// uses enum matching.
    pub fn take<T: AggResultData>(&mut self, name: &str) -> Result<T, AggAccessError> {
        let agg = self
            .data
            .remove(name)
            .ok_or_else(|| AggAccessError::AggNotFound(name.to_owned()))?;

        T::unwrap_inner(agg)
    }
}

impl<'de> Deserialize<'de> for AggResultCollection {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        // Notifies which key type that comes back from
        // ElasticSearch.  The client asks for results
        // in a way where every result is marked with
        // a `type#name` key.
        enum KeyType {
            StringTerms(String),
            NumbericTerms(String),
        }

        impl<'de> Deserialize<'de> for KeyType {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                struct TypeVisitor;

                impl<'de> Visitor<'de> for TypeVisitor {
                    type Value = KeyType;

                    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                        formatter.write_str("a string tagged with 'type#name'")
                    }

                    fn visit_str<E>(self, key: &str) -> Result<Self::Value, E>
                    where
                        E: de::Error,
                    {
                        const FIELDS: &[&str] = &[NumericTerms::ES_KEY, StringTerms::ES_KEY];

                        let (agg_type, name) = key
                            .split_once('#')
                            .ok_or_else(|| de::Error::custom("'#' missing to determine type"))?;

                        match agg_type {
                            StringTerms::ES_KEY => Ok(KeyType::StringTerms(name.to_owned())),
                            NumericTerms::ES_KEY => Ok(KeyType::NumbericTerms(name.to_owned())),
                            unknown => Err(de::Error::unknown_variant(unknown, FIELDS)),
                        }
                    }
                }

                deserializer.deserialize_str(TypeVisitor)
            }
        }

        struct AggResultVisitor;

        impl<'de> Visitor<'de> for AggResultVisitor {
            type Value = AggResultCollection;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter
                    .write_str("key-value pairs of aggregations where the keys are tagged by type")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: MapAccess<'de>,
            {
                let mut results = AggResultCollection::default();

                while let Some(agg_key) = map.next_key()? {
                    match agg_key {
                        KeyType::StringTerms(name) => {
                            let agg: StringTerms = map.next_value()?;
                            results.data.insert(name, agg.into());
                        }
                        KeyType::NumbericTerms(name) => {
                            let agg: NumericTerms = map.next_value()?;
                            results.data.insert(name, agg.into());
                        }
                    }
                }

                Ok(results)
            }
        }

        deserializer.deserialize_map(AggResultVisitor)
    }
}
