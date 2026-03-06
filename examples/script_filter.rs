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
    let not_even_by_a_dollar =
        script_filter("doc['cost'].value % params.divisor != 0").with_params([("divisor", 100)]);
    search.with(not_even_by_a_dollar);
    let mut results = client.search::<InventoryItem>(&search).await?;
    let docs = results.docs_take();
    assert!(!docs.is_empty());
    assert_eq!(docs.len(), 1);
    assert_eq!(docs.first().unwrap().cost, 120);
    Ok(())
}
