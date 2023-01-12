use serde::Serialize;

/// Direction to sort results
#[derive(Debug, Copy, Clone)]
pub enum SortDirection {
    /// smallest to biggest
    Ascending,
    /// biggest to smallest
    Descending,
}

impl Serialize for SortDirection {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Self::Ascending => "asc".serialize(serializer),
            Self::Descending => "desc".serialize(serializer),
        }
    }
}
