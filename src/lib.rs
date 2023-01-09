//!
//! ElasticLens
//!
//! > An opinionated framework to work with Elasticsearch
//!

// This stops the dead_code and unused_imports warnings from
// cropping up durring the development of code.  These warnings
// will and should still crop up during a release build
#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
// This ensures a higher level of hygiene for a production build
#![cfg_attr(
    not(debug_assertions),
    deny(
        missing_docs,
        missing_debug_implementations,
        missing_copy_implementations,
        trivial_casts,
        trivial_numeric_casts,
        unsafe_code,
        unstable_features,
    )
)]

pub mod client;
pub mod request;
pub mod response;

mod errors;
pub use errors::*;

/// If you `use elastic_lens::prelude::*` you will bring into
/// scope most of the functionality needed to get a client
/// setup, create requests, and parse results.
pub mod prelude {
    pub use crate::client::Client;
    pub use crate::request::search::{
        AggregationBuilder, CriteriaBuilder, IntoGeoPoint, Search, SubAggregationBuilder,
    };
    pub use crate::response::{NumericTerms, Stats, StringTerms};
}
