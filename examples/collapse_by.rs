mod inventory_item;

use elastic_lens::prelude::*;
use elastic_lens::Error;
use inventory_item::*;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let client = Client::default_builder()
        .host("http://localhost:9200")
        .index("inventory")
        .build()?;

    let mut search = Search::default();
    search
        .collapse_by_field(CATEGORY)
        .max_concurrent_searches(3);
    search.set_limit(3);

    let results = client.search::<InventoryItem>(&search).await?;
    assert_eq!(results.hits().count(), 2);

    Ok(())
}
