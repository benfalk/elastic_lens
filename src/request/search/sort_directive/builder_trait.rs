use super::*;
use std::borrow::Cow;

mod field_sort_builder;
mod field_sort_builder_options;
mod geo_distance_sort_builder;
mod script_score_builder;
mod script_sort_builder;

pub use field_sort_builder::*;
pub use field_sort_builder_options::*;
pub use geo_distance_sort_builder::*;
pub use script_score_builder::*;
pub use script_sort_builder::*;

/// Use a field to sort by, this can be just the field in a
/// normal direction sort or some kind of distance sort.
pub fn by_field<F: Into<Field>>(field: F) -> FieldSortBuilder {
    FieldSortBuilder {
        field: field.into(),
    }
}

/// Use to sort by a script-score.
pub fn by_script_score<S: Into<Cow<'static, str>>>(script: S) -> ScriptScoreBuilder {
    ScriptScoreBuilder::new(script)
}

/// Use to sort by a script
pub fn by_script<S: Into<Cow<'static, str>>>(script: S) -> ScriptSortBuilder {
    ScriptSortBuilder::new(script)
}

/// Trait used to provide self-sort construction
pub trait SortBuilderTrait {
    /// mutable reference to sorts you want to build on
    fn sort_directives_mut(&mut self) -> &mut Vec<SortDirective>;

    /// Add a directive by which to sort results.
    fn sort<S: Into<SortDirective>>(&mut self, sort: S) {
        self.sort_directives_mut().push(sort.into());
    }
}

impl SortBuilderTrait for Vec<SortDirective> {
    fn sort_directives_mut(&mut self) -> &mut Vec<SortDirective> {
        self
    }
}
