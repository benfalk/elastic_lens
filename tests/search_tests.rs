use elastic_lens::prelude::*;
use elastic_lens::request::search::SearchTrait;
use serde_json::json;

#[test]
fn a_default_search_is_empty() {
    let search = Search::default();

    assert_eq!(
        json!({}),
        serde_json::to_value(search.search_body()).unwrap()
    );
}

#[test]
fn a_search_with_a_limit() {
    let mut search = Search::default();
    search.set_limit(42);

    assert_eq!(
        json!({
            "size": 42
        }),
        serde_json::to_value(search.search_body()).unwrap()
    );
}

#[test]
fn a_search_with_a_limit_and_offset() {
    let mut search = Search::default();
    search.set_limit(42);
    search.set_offset(13);

    assert_eq!(
        json!({
            "size": 42,
            "from": 13,
        }),
        serde_json::to_value(search.search_body()).unwrap()
    );
}

#[test]
fn a_search_with_a_positive_contains() {
    let mut search = Search::default();
    search.field("foo").contains("bar");

    assert_eq!(
        serde_json::to_value(search.search_body()).unwrap(),
        json!({
            "query": {
                "bool": {
                    "filter": [
                        { "term": { "foo": "bar" } }
                    ]
                }
            }
        })
    );
}

#[test]
fn a_search_with_a_negative_contains() {
    let mut search = Search::default();
    search.field("foo").not().contains("bar");

    assert_eq!(
        serde_json::to_value(search.search_body()).unwrap(),
        json!({
            "query": {
                "bool": {
                    "must_not": [
                        { "term": { "foo": "bar" } }
                    ]
                }
            }
        })
    );
}

#[test]
fn a_search_with_both_negative_and_positive_contains() {
    let mut search = Search::default();
    search.field("foo").not().contains("bar");
    search.field("foo").contains("biz");

    assert_eq!(
        serde_json::to_value(search.search_body()).unwrap(),
        json!({
            "query": {
                "bool": {
                    "filter": [
                        { "term": { "foo": "biz" } }
                    ],
                    "must_not": [
                        { "term": { "foo": "bar" } }
                    ]
                }
            }
        })
    );
}
