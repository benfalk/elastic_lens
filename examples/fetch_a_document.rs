mod inventory_item;

use elastic_lens::errors::Error;
use elastic_lens::prelude::*;
use inventory_item::InventoryItem;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let client = Client::default_builder()
        .host("http://localhost:9200")
        .index("inventory")
        .build()?;

    let id = std::env::args()
        .skip(1)
        .next()
        .unwrap_or_else(|| "1".to_owned());

    if let Some(doc) = client.get_by_id::<InventoryItem>(&id).await? {
        println!("{doc:?}");
    } else {
        println!("Not Found");
    }

    Ok(())
}
