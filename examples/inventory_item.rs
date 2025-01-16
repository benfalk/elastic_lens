use elastic_lens::pub_field;
use serde::Deserialize;

pub_field!(CATEGORY, "category");
pub_field!(SUB_CATEGORY, "sub_category");
pub_field!(ACTIVE, "active");
pub_field!(COST, "cost");

#[allow(dead_code)]
#[derive(Debug, Deserialize, Clone)]
pub struct InventoryItem {
    pub category: String,
    pub sub_category: String,
    pub active: bool,
    pub cost: usize,
}
