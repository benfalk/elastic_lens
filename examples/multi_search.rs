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

    let mut clothing = Search::default();
    clothing.field("category").contains("clothing");

    let mut office = Search::default();
    office.field("category").contains("office");

    let results = client
        .multi_search::<InventoryItem>(&[clothing, office])
        .await?;

    println!("Clothing:");

    for doc in results[0].docs() {
        println!("{doc:?}");
    }

    println!("\nOffice:");

    for doc in results[1].docs() {
        println!("{doc:?}");
    }

    Ok(())
}
