use serde::Serialize;

use super::*;

/// Describes how to collapse search results
#[derive(Debug, Clone, Serialize)]
pub struct CollapseBy {
    field: Field,

    #[serde(rename = "max_concurrent_group_searches")]
    #[serde(skip_serializing_if = "Option::is_none")]
    max_concurrent: Option<u16>,
}

/// Update and define how searches should be collappsed
#[derive(Debug)]
pub struct CollapseBuilder<'a> {
    writer: &'a mut CollapseBy,
}

impl CollapseBuilder<'_> {
    /// How many searches can be run in parallel at any
    /// given time to populate inner hits of a collappsed
    /// search taking place
    pub fn max_concurrent_searches(&mut self, amount: u16) {
        self.writer.max_concurrent = Some(amount);
    }
}

/// Functionality to "group by" document results together
///
/// https://www.elastic.co/guide/en/elasticsearch/reference/7.17/collapse-search-results.html
pub trait CollapseBuilderTrait {
    /// Mutable reference to an optional [CollapseBy]
    fn collapse_mut(&mut self) -> &mut Option<CollapseBy>;

    /// Creates a builder to begin collapsing documents
    fn collapse_by_field<F>(&mut self, field: F) -> CollapseBuilder<'_>
    where
        F: Into<Field>,
    {
        let writer = self.collapse_mut();

        *writer = Some(CollapseBy {
            field: field.into(),
            max_concurrent: None,
        });

        CollapseBuilder {
            writer: writer.as_mut().unwrap(),
        }
    }
}
