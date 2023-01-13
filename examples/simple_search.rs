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

    search.with(field("category").contains("clothing"));
    search.with(!field("sub_category").contains("beanie"));
    search.with(field("cost").between(1000..2000));

    search.set_limit(10);

    let results = client.search::<InventoryItem>(&search).await?;
    println!("{:?}", results.count());
    for doc in results.docs() {
        println!("{doc:?}");
    }

    Ok(())
}
