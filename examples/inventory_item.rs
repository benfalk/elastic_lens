use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct InventoryItem {
    pub category: String,
    pub sub_category: String,
    pub active: bool,
    pub cost: usize,
}
