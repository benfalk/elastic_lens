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

    let script_sort = by_script(
        r#"
            if ( doc['cost'].value > params.breakpoint ) {
                doc['cost'].value / 100
            } else {
                doc['cost'].value * 100
            }
        "#,
    )
    .with_params([("breakpoint", 1300)])
    .returns_a_number()
    .ascending();

    let mut search = Search::default();
    search.sort(script_sort);
    let mut results = client.search::<InventoryItem>(&search).await?;
    let docs = results.docs_take();
    assert!(!docs.is_empty());
    Ok(())
}
