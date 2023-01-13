mod inventory_item;

pub mod create_client {
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
}

pub mod create_an_run_search {
    use super::inventory_item::*;
    use crate::create_client::create_client;
    use elastic_lens::{prelude::*, response::SearchResults, Error};

    pub async fn clothing_inventory() -> Result<SearchResults<InventoryItem>, Error> {
        let client = create_client()?;

        let mut search = Search::default();
        search.with(CATEGORY.contains("clothing"));
        search.with(COST.greater_than(500));

        Ok(client.search(&search).await?)
    }
}

pub mod complex_search {
    use crate::create_client::create_client;
    use elastic_lens::{prelude::*, request::search::GeoPoint, response::SearchResults, Error};
    use serde_json::Value;

    // Getting the documents back as serde JSON
    // The client will deserialize the `_source` to any type
    // that implements for it.  See `examples/inventory_item.rs`
    // for an example.
    pub async fn complex_search() -> Result<SearchResults<Value>, Error> {
        let client = create_client()?;

        let mut search = Search::default();

        search.with(
            field("server.location")
                .within(500)
                .miles()
                .of(GeoPoint::new(12.2, 18.9)),
        );

        search.with(field("log.level").has_any_of(["error", "warning"]));
        search.with(field("log.trace").exists());

        search.with(if_any_match(|any| {
            any.with(field("service").contains("backend-core"));

            any.with(if_all_match(|all| {
                all.with(field("service").contains("frontend-core"));
                all.with(field("tags").has_any_of(["market-place", "registration"]));
            }));
        }));

        Ok(client.search(&search).await?)
    }
}

pub mod terms_aggregations {
    use crate::create_client::create_client;
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
}

pub mod stats_aggregation {
    use crate::create_client::create_client;
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
}

pub mod filter_aggregation {
    use super::inventory_item::*;
    use crate::create_client::create_client;
    use elastic_lens::{prelude::*, Error};

    pub async fn less_than_20_report() -> Result<Filtered, Error> {
        let client = create_client()?;

        let mut search = Search::default();

        search
            .create_aggregation("under-twenty")
            .filtered_by(|search| search.with(COST.less_than(20_00)))
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
}

pub mod multi_search {
    use super::inventory_item::*;
    use crate::create_client::create_client;
    use elastic_lens::{prelude::*, Error};

    pub async fn report_clothing_and_office() -> Result<(), Error> {
        let client = create_client()?;

        let mut clothing = Search::default();
        clothing.with(CATEGORY.contains("clothing"));

        let mut office = Search::default();
        office.with(CATEGORY.contains("office"));

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
}

pub mod geo_sort {
    use crate::create_client::create_client;
    use elastic_lens::{prelude::*, request::search::GeoPoint, response::SearchResults, Error};
    use serde_json::Value;

    pub async fn nearest_allies() -> Result<SearchResults<Value>, Error> {
        let client = create_client()?;

        let mut search = Search::default();

        search.with(field("user.is_ally").contains(true));

        search
            .sort_field("user.location")
            .by_distance_from(GeoPoint::new(1.1, 2.2))
            .in_ascending_order()
            .ignore_unmapped_documents();

        Ok(client.search(&search).await?)
    }
}

pub mod five_cheapest {
    use crate::create_client::create_client;
    use elastic_lens::{prelude::*, response::SearchResults, Error};
    use serde_json::Value;

    pub async fn five_cheapest_items() -> Result<SearchResults<Value>, Error> {
        let client = create_client()?;
        let mut search = Search::default();

        search
            .sort_field("cost")
            .ascending()
            .with_missing_values_last();

        search.set_limit(5);

        Ok(client.search(&search).await?)
    }
}
