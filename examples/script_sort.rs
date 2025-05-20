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
    .with_params([("breakpoint", 1300)]);

    let mut decending_search = Search::default();
    decending_search.sort(script_sort.clone().decending());

    let mut ascending_search = Search::default();
    decending_search.sort(script_sort.ascending());

    let mut decending = client.search::<InventoryItem>(&decending_search).await?;
    let mut ascending = client.search::<InventoryItem>(&ascending_search).await?;

    let acending_docs = ascending.docs_take();
    let mut decending_docs = decending.docs_take();
    decending_docs.reverse();
    decending_docs.reverse();

    assert!(!decending_docs.is_empty());
    assert_eq!(acending_docs, decending_docs);
    Ok(())
}
