# Elastic Lens

> An opinionated framework to work with Elasticsearch.

## About

Simple DSL framework to build Elasticsearch requests as well
as parse the responses back.  There is a **STRONG** chance not
every piece of functionality is available so read the docs and
make sure it's the right fit for you.

This project is in it's infancy and is currently supporting a
real work project.  This is what is driving it's development
for now; however, if you have suggestions or edits please feel
free to open an issue :+1:.

## Functionality Tour

All of these samples can also be found in `examples/sample_code.rs`.

### Creating a Client

```rust
use elastic_lens::{
    client::{Client, DefaultAdapter},
    Error,
};

pub fn create_client() -> Result<Client<DefaultAdapter>, Error> {
    Ok(Client::default_builder()
        .host("http://localhost:9200")
        .index("sample-index")
        .credentials("username", "secret")
        .default_limit(20)
        .build()?)
}
```

### Simple Search

```rust
// See `examples/inventory_item.rs` for definition
use super::inventory_item::InventoryItem;

use elastic_lens::{prelude::*, response::SearchResults, Error};

pub async fn clothing_inventory() -> Result<SearchResults<InventoryItem>, Error> {
    let client = create_client()?;

    let mut search = Search::default();
    search.field("category").contains("clothing");
    search.field("cost").greater_than(500);

    Ok(client.search(&search).await?)
}
```

### Complex Search

```rust
use elastic_lens::{prelude::*, request::search::GeoPoint, response::SearchResults, Error};
use serde_json::Value;

// Getting the documents back as serde JSON
// The client will deserialize the `_source` to any type
// that implements for it.  See `examples/inventory_item.rs`
// for an example.
pub async fn complex_search() -> Result<SearchResults<Value>, Error> {
    let client = create_client()?;
    let mut search = Search::default();

    search
        .field("server.location")
        .within(500)
        .miles()
        .of(GeoPoint::new(12.2, 18.9));

    search.field("log.level").any_of(["error", "warning"]);
    search.field("log.trace").exists();

    search.if_any_match(|any| {
        any.field("service").contains("backend-core");

        any.if_all_match(|all| {
            all.field("service").contains("frontend-core");
            all.field("tags").any_of(["market-place", "registration"]);
        });
    });

    Ok(client.search(&search).await?)
}
```

### MultiSearch

```rust
use super::inventory_item::InventoryItem;
use elastic_lens::{prelude::*, Error};

pub async fn report_clothing_and_office() -> Result<(), Error> {
    let client = create_client()?;

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
```

### Term Aggregations

```rust
use elastic_lens::{prelude::*, Error};
use serde_json::Value;

pub async fn category_aggs_with_sub_categories() -> Result<StringTerms, Error> {
    let client = create_client()?;
    let mut search = Search::default();

    search
        .create_aggregation("categories")
        .for_field("category")
        .count_terms()
        .with_sub_aggregations(|sub| {
            sub.create_aggregation("sub-categories")
                .for_field("sub_category")
                .count_terms()
                .for_top(20);
        });

    let mut results = client.search::<Value>(&search).await?;
    Ok(results.aggs_mut().take("categories")?)
}
```

### Stats Aggregations

```rust
use elastic_lens::{prelude::*, Error};
use serde_json::Value;

pub async fn collect_price_stats() -> Result<Stats, Error> {
    let client = create_client()?;
    let mut search = Search::default();

    search
        .create_aggregation("price-stats")
        .for_field("item.price")
        .collect_stats();

    let mut results = client.search::<Value>(&search).await?;
    Ok(results.aggs_mut().take("price-stats")?)
}
```

### Filter Aggregations

```rust
use elastic_lens::{prelude::*, Error};

pub async fn less_than_20_report() -> Result<Filtered, Error> {
    let client = create_client()?;

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
    Ok(results.aggs_mut().take::<Filtered>("under-twenty")?)
}
```

## Playing with the Examples

You'll need Elasticsearch running on port 9200. You can run it dockerized with:

```
docker run -it -p 9200:9200 -e "discovery.type=single-node" elasticsearch:7.11.2
```

Then, run `bin/setup_index.sh` to bootstrap some data that the examples will query against.

The examples are found in the `examples/` directory and can be
run with cargo: `cargo run --example <name> (..args..)`  Here is
current list of examples:

- `fetch_a_document`
- `simple_search`
- `simple_aggs`
