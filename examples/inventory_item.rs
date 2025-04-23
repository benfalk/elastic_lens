use elastic_lens::pub_field;
use serde::Deserialize;

pub_field!(CATEGORY, "category");
pub_field!(SUB_CATEGORY, "sub_category");
pub_field!(ACTIVE, "active");
pub_field!(COST, "cost");
pub_field!(VENDORS, "vendors");
pub_field!(VENDOR_SLUG, "vendors.slug");
pub_field!(VENDOR_COUNTRY, "vendors.country");

#[derive(Debug, Deserialize, Clone)]
pub struct InventoryItem {
    pub category: String,
    pub sub_category: String,
    pub active: bool,
    pub cost: usize,
    pub vendors: Vec<Vendor>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Vendor {
    pub slug: String,
    pub country: String,
}
