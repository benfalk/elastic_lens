use elastic_lens::response::{
    AggResult, AggResultCollection, NumericTerms, Stats, StringTerms, TermCount,
};
use serde_json::json;

fn to_json<S: serde::Serialize>(value: S) -> serde_json::Value {
    serde_json::to_value(value).unwrap()
}

fn round_trip<T>(value: &T) -> T
where
    T: serde::Serialize + serde::de::DeserializeOwned + std::fmt::Debug,
{
    let json = to_json(value);
    serde_json::from_value(json).expect("value should deserialize after serialization")
}

#[test]
fn stats_serializes_to_elasticsearch_shape() {
    let stats = Stats {
        count: 10,
        min: Some(1.0),
        max: Some(100.0),
        avg: Some(50.0),
        sum: 500.0,
    };

    assert_eq!(
        to_json(stats),
        json!({
            "count": 10,
            "min": 1.0,
            "max": 100.0,
            "avg": 50.0,
            "sum": 500.0
        })
    );
}

#[test]
fn stats_round_trips_through_json() {
    let stats = Stats {
        count: 3,
        min: None,
        max: Some(42.0),
        avg: Some(21.0),
        sum: 63.0,
    };

    let restored = round_trip(&stats);

    assert_eq!(restored.count, stats.count);
    assert_eq!(restored.min, stats.min);
    assert_eq!(restored.max, stats.max);
    assert_eq!(restored.avg, stats.avg);
    assert_eq!(restored.sum, stats.sum);
}

#[test]
fn string_terms_serializes_with_sub_aggregations() {
    let mut sub_aggs = AggResultCollection::default();
    sub_aggs.insert(
        "price_stats",
        AggResult::Stats(Stats {
            count: 10,
            min: Some(1.0),
            max: Some(100.0),
            avg: Some(50.0),
            sum: 500.0,
        }),
    );

    let terms = StringTerms {
        terms: vec![TermCount {
            term: "foo".to_owned(),
            count: 10,
            sub_aggs,
        }],
        other_doc_count: 5,
        doc_count_error_upper_bound: 0,
    };

    assert_eq!(
        to_json(terms),
        json!({
            "buckets": [
                {
                    "key": "foo",
                    "doc_count": 10,
                    "stats#price_stats": {
                        "count": 10,
                        "min": 1.0,
                        "max": 100.0,
                        "avg": 50.0,
                        "sum": 500.0
                    }
                }
            ],
            "sum_other_doc_count": 5,
            "doc_count_error_upper_bound": 0
        })
    );
}

#[test]
fn string_terms_round_trips_through_json() {
    let mut sub_aggs = AggResultCollection::default();
    sub_aggs.insert(
        "price_stats",
        AggResult::Stats(Stats {
            count: 2,
            min: Some(5.0),
            max: Some(15.0),
            avg: Some(10.0),
            sum: 20.0,
        }),
    );

    let terms = StringTerms {
        terms: vec![
            TermCount {
                term: "alpha".to_owned(),
                count: 7,
                sub_aggs: AggResultCollection::default(),
            },
            TermCount {
                term: "beta".to_owned(),
                count: 3,
                sub_aggs,
            },
        ],
        other_doc_count: 12,
        doc_count_error_upper_bound: 1,
    };

    let restored = round_trip(&terms);

    assert_eq!(restored.terms.len(), 2);
    assert_eq!(restored.terms[0].term, "alpha");
    assert_eq!(restored.terms[0].count, 7);
    assert_eq!(restored.terms[1].term, "beta");
    assert_eq!(restored.terms[1].count, 3);
    assert_eq!(restored.other_doc_count, 12);
    assert_eq!(restored.doc_count_error_upper_bound, 1);

    let sub_stats = restored.terms[1]
        .sub_aggs
        .get::<Stats>("price_stats")
        .unwrap();
    assert_eq!(sub_stats.count, 2);
    assert_eq!(sub_stats.sum, 20.0);
}

#[test]
fn numeric_terms_round_trips_through_json() {
    let terms = NumericTerms {
        terms: vec![TermCount {
            term: 42,
            count: 9,
            sub_aggs: AggResultCollection::default(),
        }],
        other_doc_count: 0,
        doc_count_error_upper_bound: 0,
    };

    let restored = round_trip(&terms);

    assert_eq!(restored.terms.len(), 1);
    assert_eq!(restored.terms[0].term, 42);
    assert_eq!(restored.terms[0].count, 9);
}
