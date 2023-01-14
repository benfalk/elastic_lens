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

    search.with(!SUB_CATEGORY.contains("beanie"));

    search.sort(
        by_script(
            r#"
              if ( doc['cost'].value > params.breakpoint ) {
                  doc['cost'].value / 100
              } else {
                  doc['cost'].value * 100
              }
            "#,
        )
        .with_params([("breakpoint", 1300)]),
    );

    let results = client.search::<InventoryItem>(&search).await?;

    println!("{:?}", results.count());

    for doc in results.docs() {
        println!("{doc:?}");
    }

    Ok(())
}
