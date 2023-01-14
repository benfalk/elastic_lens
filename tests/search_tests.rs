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
    search.with(field("foo").contains("bar"));

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
    search.with(!field("foo").contains("bar"));

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
    search.with(!field("foo").contains("bar"));
    search.with(field("foo").contains("biz"));

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
    search.with(field("cost").less_than(800));

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
    search.with(field("cost").less_than_or_equal(800));

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
    search.with(field("cost").greater_than(800));

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
    search.with(field("cost").greater_than_or_equal(800));

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
    search.with(field("cost").between(200..350));

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
    search.with(field("category").has_any_of(["clothing", "office"]));
    search.with(!field("rankings").has_any_of([1, 2, 3]));

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
    search.with(field("user.hobbies").exists());
    search.with(!field("user.dislikes").exists());

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
    search.with(
        field("user.address.geo-point")
            .within(50)
            .miles()
            .of(GeoPoint::new(1.1, 2.2)),
    );

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
    search.with(
        !field("user.address.geo-point")
            .within(50)
            .kilomenters()
            .of(GeoPoint::new(1.1, 2.2)),
    );

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

    search.with(if_any_match(|critera| {
        critera.with(field("user.role").contains("admin"));
        critera.with(field("user.score").greater_than(9000));
    }));

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

    search.with(if_any_match(|critera| {
        critera.with(field("user.role").contains("admin"));
        critera.with(field("user.score").greater_than(9000));
        critera.with(!field("user.hobbies").has_any_of(["swimming", "tennis"]));
    }));

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

    search.with(if_any_match(|q| {
        q.with(field("user.role").contains("admin"));

        q.with(if_all_match(|a| {
            a.with(!field("user.banned").contains(true));
            a.with(field("user.group").has_any_of(["rust", "fps"]));
        }));
    }));

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

#[test]
fn building_a_search_with_term_aggregations() {
    let mut search = Search::default();

    search
        .create_aggregation("top-categories")
        .for_field("category")
        .count_terms();

    assert_eq!(
        search_to_json(search),
        json!({
            "aggs": {
                "top-categories": {
                    "terms": { "field": "category" }
                }
            }
        })
    );
}

#[test]
fn building_a_search_with_term_aggregations_and_sub_aggregations() {
    let mut search = Search::default();

    search
        .create_aggregation("top-categories")
        .for_field("category")
        .count_terms()
        .with_sub_aggregations(|aggs| {
            aggs.create_aggregation("sub-categories")
                .for_field("sub_category")
                .count_terms()
                .for_top(20);
        });

    assert_eq!(
        search_to_json(search),
        json!({
            "aggs": {
                "top-categories": {
                    "terms": { "field": "category" },
                    "aggs": {
                        "sub-categories": {
                            "terms": { "field": "sub_category", "size": 20 }
                        }
                    }
                }
            }
        })
    );
}

#[test]
fn building_a_search_with_stats_aggregations() {
    let mut search = Search::default();

    search
        .create_aggregation("price-stats")
        .for_field("item.price")
        .collect_stats();

    assert_eq!(
        search_to_json(search),
        json!({
            "aggs": {
                "price-stats": {
                    "stats": { "field": "item.price" }
                }
            }
        })
    );
}

#[test]
fn building_a_search_with_stats_a_missing_value() {
    let mut search = Search::default();

    search
        .create_aggregation("test-score-stats")
        .for_field("test.score")
        .collect_stats()
        .use_for_missing_value(0);

    assert_eq!(
        search_to_json(search),
        json!({
            "aggs": {
                "test-score-stats": {
                    "stats": { "field": "test.score", "missing": 0 }
                }
            }
        })
    );
}

#[test]
fn building_a_search_with_a_geo_sort() {
    let mut search = Search::default();
    use elastic_lens::request::search::GeoPoint;

    search.sort(
        by_field("user.location")
            .by_distance_from(GeoPoint::default())
            .using_the_plane_formula()
            .in_ascending_order()
            .ignore_unmapped_documents(),
    );

    assert_eq!(
        search_to_json(search),
        json!({
            "sort": [
                {
                    "_geo_distance": {
                        "user.location": { "lat": 0.0, "lon": 0.0 },
                        "order": "asc",
                        "distance_type": "plane",
                        "ignore_unmapped": true
                    }
                }
            ]
        })
    );
}

#[test]
fn building_a_search_with_a_normal_field_sort() {
    let mut search = Search::default();

    search.sort(by_field("user.age").descending());

    assert_eq!(
        search_to_json(search),
        json!({
            "sort": [
                { "user.age": "desc" }
            ]
        })
    );
}

#[test]
fn building_a_search_with_a_normal_field_sort_and_missing_value() {
    let mut search = Search::default();

    search.sort(by_field("user.age").ascending().where_missing_use(42));

    assert_eq!(
        search_to_json(search),
        json!({
            "sort": [
                { "user.age": { "order": "asc", "missing": 42 } }
            ]
        })
    );
}

#[test]
fn building_a_search_with_a_script_sort() {
    let mut search = Search::default();

    search.with(field("user.role").contains("admin"));
    search.sort(by_script("doc['my-int'].value / 10"));

    assert_eq!(
        search_to_json(search),
        json!({
            "query": {
                "script_score": {
                    "query": {
                        "bool": {
                            "filter": [
                                { "term": { "user.role": "admin" } }
                            ]
                        }
                    },
                    "script": {
                        "source": "doc['my-int'].value / 10"
                    }
                }
            }
        })
    );
}

#[test]
fn building_a_search_with_a_script_sort_and_params() {
    let mut search = Search::default();

    search.with(field("user.role").contains("admin"));
    search.sort(by_script("doc['my-int'].value / 10 + params.spice").with_params([("spice", 42)]));

    assert_eq!(
        search_to_json(search),
        json!({
            "query": {
                "script_score": {
                    "query": {
                        "bool": {
                            "filter": [
                                { "term": { "user.role": "admin" } }
                            ]
                        }
                    },
                    "script": {
                        "source": "doc['my-int'].value / 10 + params.spice",
                        "params": {
                            "spice": 42
                        }
                    }
                }
            }
        })
    );
}
