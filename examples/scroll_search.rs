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

    search.with(CATEGORY.contains("clothing"));
    search.with(!SUB_CATEGORY.contains("beanie"));
    search.with(COST.between(1000..2000));
    search.set_limit(1);

    let (mut cursor, results) = client.scroll_search::<InventoryItem>(&search).await?;
    assert_eq!(results.hits().count(), 1);

    let results = client.scroll::<InventoryItem>(&mut cursor).await?;
    assert_eq!(results.hits().count(), 1);

    let results = client.scroll::<InventoryItem>(&mut cursor).await?;
    assert_eq!(results.hits().count(), 0);

    Ok(())
}
