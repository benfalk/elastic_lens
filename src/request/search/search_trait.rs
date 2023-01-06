use super::*;

/// This is the trait that is used by the client
/// to build a search request to send to Elasticsearch.
/// By default this trait is an empty search and will
/// return whatever a naked search would.
///
/// If you want to find an implementation look at `Search`.
pub trait SearchTrait: Sized {
    /// How many documents to limit by, None will use the default
    fn limit(&self) -> Option<usize> {
        None
    }

    /// What offset to use, None is the same as zero
    fn offset(&self) -> Option<usize> {
        None
    }

    /// the criteria that will select documents
    fn positive_criteria(&self) -> Option<&Vec<Criterion>> {
        None
    }

    /// the criteria that will exclude documents
    fn negative_criteria(&self) -> Option<&Vec<Criterion>> {
        None
    }

    /// Produces a structure that can be serialized into the body
    /// request for Elasticsearch.  This is a borrow from the trait
    /// and therefore locks modification while the body is around.
    fn search_body(&self) -> SearchBody<'_> {
        SearchBody::from(self)
    }
}
