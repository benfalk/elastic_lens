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

    search.set_limit(10);

    let results = client.search::<InventoryItem>(&search).await?;
    println!("{:?}", results.count());
    for doc in results.docs() {
        println!("{doc:?}");
    }

    Ok(())
}
