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

#[test]
fn a_search_with_geo_distance_in_miles() {
    use elastic_lens::request::search::GeoPoint;

    let mut search = Search::default();
    search
        .field("user.address.geo-point")
        .within(50)
        .miles()
        .of(GeoPoint::new(1.1, 2.2));

    assert_eq!(
        search_to_json(search),
        json!({
            "query": {
                "bool": {
                    "filter": [
                        {
                            "geo_distance": {
                                "distance": "50mi",
                                "user.address.geo-point": {
                                    "lat": 1.1,
                                    "lon": 2.2,
                                }
                            }
                        }
                    ]
                }
            }
        })
    );
}

#[test]
fn a_search_with_geo_distance_in_kilometers_in_not() {
    use elastic_lens::request::search::GeoPoint;

    let mut search = Search::default();
    search
        .field("user.address.geo-point")
        .not()
        .within(50)
        .kilomenters()
        .of(GeoPoint::new(1.1, 2.2));

    assert_eq!(
        search_to_json(search),
        json!({
            "query": {
                "bool": {
                    "must_not": [
                        {
                            "geo_distance": {
                                "distance": "50km",
                                "user.address.geo-point": {
                                    "lat": 1.1,
                                    "lon": 2.2,
                                }
                            }
                        }
                    ]
                }
            }
        })
    );
}

#[test]
fn a_search_with_simple_if_any_match() {
    let mut search = Search::default();

    search.if_any_match(|critera| {
        critera.field("user.role").contains("admin");
        critera.field("user.score").greater_than(9000);
    });

    assert_eq!(
        search_to_json(search),
        json!({
            "query": {
                "bool": {
                    "filter": [
                        {
                            "bool": {
                                "should": [
                                    { "term": { "user.role": "admin" } },
                                    { "range": { "user.score": { "gt": 9000 } } }
                                ]
                            }
                        }
                    ]
                }
            }
        })
    );
}

#[test]
fn a_search_with_not_in_if_any_match() {
    let mut search = Search::default();

    search.if_any_match(|critera| {
        critera.field("user.role").contains("admin");
        critera.field("user.score").greater_than(9000);
        critera
            .field("user.hobbies")
            .not()
            .any_of(["swimming", "tennis"]);
    });

    assert_eq!(
        search_to_json(search),
        json!({
            "query": {
                "bool": {
                    "filter": [
                        {
                            "bool": {
                                "should": [
                                    { "term": { "user.role": "admin" } },
                                    { "range": { "user.score": { "gt": 9000 } } },
                                    {
                                        "bool": {
                                            "must_not": [
                                                { "terms": { "user.hobbies": ["swimming", "tennis" ] } }
                                            ]
                                        }
                                    }
                                ]
                            }
                        }
                    ]
                }
            }
        })
    );
}

#[test]
fn a_search_using_if_all_match_inside_if_any_match() {
    let mut search = Search::default();

    search.if_any_match(|q| {
        q.field("user.role").contains("admin");

        q.if_all_match(|a| {
            a.field("user.banned").not().contains(true);
            a.field("user.group").any_of(["rust", "fps"]);
        });
    });

    assert_eq!(
        search_to_json(search),
        json!({
            "query": {
                "bool": {
                    "filter": [
                        {
                            "bool": {
                                "should": [
                                    { "term": { "user.role": "admin" } },
                                    {
                                        "bool": {
                                            "filter": [
                                                { "terms": { "user.group": ["rust", "fps"] } }
                                            ],
                                            "must_not": [
                                                { "term": { "user.banned": true } }
                                            ]
                                        }
                                    }
                                ]
                            }
                        }
                    ]
                }
            }
        })
    );
}
