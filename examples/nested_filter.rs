mod inventory_item;

use elastic_lens::prelude::*;
use elastic_lens::Error;
use inventory_item::{InventoryItem, VENDORS, VENDOR_COUNTRY, VENDOR_SLUG};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let client = Client::default_builder()
        .host("http://localhost:9200")
        .index("inventory")
        .build()?;

    let mut search = Search::default();

    // Find inventory sold at the illusive Canadian K-Mart
    search.with(nested(VENDORS, |criteria| {
        criteria.with(VENDOR_SLUG.contains("kmart"));
        criteria.with(VENDOR_COUNTRY.contains("ca"));
    }));

    let results = client.search::<InventoryItem>(&search).await?;
    assert_eq!(results.docs().count(), 1);
    assert_eq!(results.docs().next().unwrap().sub_category, "beanie");

    Ok(())
}
