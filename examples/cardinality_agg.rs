use elastic_lens::prelude::*;
use elastic_lens::Error;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let client = Client::default_builder()
        .host("http://localhost:9200")
        .index("inventory")
        .build()?;

    let mut search = Search::default();

    search
        .create_aggregation("category_count")
        .for_field("category")
        .count_cardinality()
        .use_for_missing_value("unknown")
        .with_precision_threshold(1000);

    // we don't care about documents
    search.set_limit(0);

    // since we know we won't have any results we can use `()`
    // as the type since it doesn't matter
    let mut results = client.search::<()>(&search).await?;
    let count = results.aggs_mut().take::<Cardinality>("category_count")?;

    println!("{count:#?}");

    Ok(())
}
