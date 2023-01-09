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
    search
        .create_aggregation("categories")
        .for_field("category")
        .count_terms()
        .with_sub_aggregations(|aggs| {
            aggs.create_aggregation("sub-categories")
                .for_field("sub_category")
                .count_terms()
                .for_top(20);
        });

    search.set_limit(0);

    let results = client.search::<InventoryItem>(&search).await?;
    let aggs = results.aggs().get::<StringTerms>("categories")?;

    println!("{aggs:#?}");

    Ok(())
}
