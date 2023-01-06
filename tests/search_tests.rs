use elastic_lens::prelude::*;
use elastic_lens::request::search::SearchTrait;
use serde_json::json;

fn search_to_json(search: Search) -> serde_json::Value {
    serde_json::to_value(search.search_body()).unwrap()
}

#[test]
fn a_default_search_is_empty() {
    let search = Search::default();

    assert_eq!(json!({}), search_to_json(search));
}

#[test]
fn a_search_with_a_limit() {
    let mut search = Search::default();
    search.set_limit(42);

    assert_eq!(
        json!({
            "size": 42
        }),
        search_to_json(search)
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
        search_to_json(search)
    );
}

#[test]
fn a_search_with_a_positive_contains() {
    let mut search = Search::default();
    search.field("foo").contains("bar");

    assert_eq!(
        search_to_json(search),
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
        search_to_json(search),
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
        search_to_json(search),
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

#[test]
fn a_search_with_less_than_range() {
    let mut search = Search::default();
    search.field("cost").less_than(800);

    assert_eq!(
        search_to_json(search),
        json!({
            "query": {
                "bool": {
                    "filter": [
                        { "range": { "cost": { "lt": 800 } } }
                    ]
                }
            }
        })
    );
}

#[test]
fn a_search_with_less_than_or_equal_range() {
    let mut search = Search::default();
    search.field("cost").less_than_or_equal(800);

    assert_eq!(
        search_to_json(search),
        json!({
            "query": {
                "bool": {
                    "filter": [
                        { "range": { "cost": { "lte": 800 } } }
                    ]
                }
            }
        })
    );
}

#[test]
fn a_search_with_greater_than_range() {
    let mut search = Search::default();
    search.field("cost").greater_than(800);

    assert_eq!(
        search_to_json(search),
        json!({
            "query": {
                "bool": {
                    "filter": [
                        { "range": { "cost": { "gt": 800 } } }
                    ]
                }
            }
        })
    );
}

#[test]
fn a_search_with_greater_than_or_equal_range() {
    let mut search = Search::default();
    search.field("cost").greater_than_or_equal(800);

    assert_eq!(
        search_to_json(search),
        json!({
            "query": {
                "bool": {
                    "filter": [
                        { "range": { "cost": { "gte": 800 } } }
                    ]
                }
            }
        })
    );
}

#[test]
fn a_search_with_between_range() {
    let mut search = Search::default();
    search.field("cost").between(200..350);

    assert_eq!(
        search_to_json(search),
        json!({
            "query": {
                "bool": {
                    "filter": [
                        { "range": { "cost": { "gte": 200, "lte": 350 } } }
                    ]
                }
            }
        })
    );
}

#[test]
fn a_search_with_any_of() {
    let mut search = Search::default();
    search.field("category").any_of(["clothing", "office"]);
    search.field("rankings").not().any_of([1, 2, 3]);

    assert_eq!(
        search_to_json(search),
        json!({
            "query": {
                "bool": {
                    "filter": [
                        { "terms": { "category": ["clothing", "office"] } }
                    ],
                    "must_not": [
                        { "terms": { "rankings": [1, 2, 3] } }
                    ]
                }
            }
        })
    );
}

#[test]
fn a_search_with_exists() {
    let mut search = Search::default();
    search.field("user.hobbies").exists();
    search.field("user.dislikes").not().exists();

    assert_eq!(
        search_to_json(search),
        json!({
            "query": {
                "bool": {
                    "filter": [
                        { "exists": { "field": "user.hobbies" } }
                    ],
                    "must_not": [
                        { "exists": { "field": "user.dislikes" } }
                    ]
                }
            }
        })
    );
}
