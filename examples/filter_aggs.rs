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
        .create_aggregation("under-twenty")
        .filtered_by(|search| search.field("cost").less_than(20_00))
        .with_sub_aggregations(|aggs| {
            aggs.create_aggregation("categories")
                .for_field("category")
                .count_terms()
                .for_top(20);
        });

    // we don't care about documents
    search.set_limit(0);

    // since we know we won't have any results we can use `()`
    // as the type since it doesn't matter
    let mut results = client.search::<()>(&search).await?;
    let under_ten = results.aggs_mut().take::<Filtered>("under-twenty")?;

    println!("{under_ten:#?}");

    Ok(())
}
