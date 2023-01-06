use super::Field;

/// Target of an query or action
///
/// Operations can have many different targets, two of which
/// are Elasticsearch fields and the output of scripts.  Where
/// such concepts can target different concepts this attempts
/// to encapsulate them into a single idea so they can be used
/// interchangeable.
#[derive(Debug, Clone)]
pub enum Target {
    /// when targeting the mapping/field of a document
    DocField(Field)
}
