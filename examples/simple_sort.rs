mod inventory_item;

use elastic_lens::prelude::*;
use elastic_lens::Error;
use inventory_item::InventoryItem;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let client = Client::default_builder()
        .host("http://localhost:9200")
        .index("inventory")
        .build()?;

    let mut search = Search::default();

    search.sort_field("cost").ascending();

    let results = client.search::<InventoryItem>(&search).await?;

    for doc in results.docs() {
        println!("{doc:?}");
    }

    Ok(())
}
