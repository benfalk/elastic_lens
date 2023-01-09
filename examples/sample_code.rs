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
    use super::inventory_item::InventoryItem;
    use crate::create_client::create_client;
    use elastic_lens::{prelude::*, response::SearchResults, Error};

    pub async fn clothing_inventory() -> Result<SearchResults<InventoryItem>, Error> {
        let client = create_client()?;

        let mut search = Search::default();
        search.field("category").contains("clothing");
        search.field("cost").greater_than(500);

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
