use super::*;

mod field_sort_builder;
mod field_sort_builder_options;
mod geo_distance_sort_builder;

pub use field_sort_builder::*;
pub use field_sort_builder_options::*;
pub use geo_distance_sort_builder::*;

/// Trait used to provide self-sort construction
pub trait SortBuilderTrait {
    /// mutable reference to sorts you want to build on
    fn sort_directives_mut(&mut self) -> &mut Vec<SortDirective>;

    /// Target a field to sort
    fn sort_field<F: Into<Field>>(&mut self, field: F) -> FieldSortBuilder<'_> {
        FieldSortBuilder {
            field: field.into(),
            sorts: self.sort_directives_mut(),
        }
    }
}

impl SortBuilderTrait for Vec<SortDirective> {
    fn sort_directives_mut(&mut self) -> &mut Vec<SortDirective> {
        self
    }
}
