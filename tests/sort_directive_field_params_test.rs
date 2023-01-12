use elastic_lens::request::search::SortField;
use serde_json::json;

fn to_json<S: serde::Serialize>(value: S) -> serde_json::Value {
    serde_json::to_value(value).unwrap()
}

#[test]
fn just_a_field_name() {
    let sort = SortField::field("user.score").build();

    assert_eq!(to_json(sort), json!("user.score"));
}

#[test]
fn field_with_missing_last() {
    let sort = SortField::field("user.score")
        .with_missing_values_last()
        .build();

    assert_eq!(
        to_json(sort),
        json!({
            "user.score": { "missing": "_last" }
        })
    );
}

#[test]
fn field_with_missing_first() {
    let sort = SortField::field("user.score")
        .with_missing_values_first()
        .build();

    assert_eq!(
        to_json(sort),
        json!({
            "user.score": { "missing": "_first" }
        })
    );
}

#[test]
fn with_a_custom_missing_value() {
    let sort = SortField::field("user.score").where_missing_use(42).build();

    assert_eq!(
        to_json(sort),
        json!({
            "user.score": { "missing": 42 }
        })
    );
}

#[test]
fn with_a_sort_direction_of_ascending() {
    let sort = SortField::field("user.score").in_ascending_order().build();

    assert_eq!(
        to_json(sort),
        json!({
            "user.score": "asc"
        })
    );
}

#[test]
fn with_a_sort_direction_of_descending() {
    let sort = SortField::field("user.score").in_descending_order().build();

    assert_eq!(
        to_json(sort),
        json!({
            "user.score": "desc"
        })
    );
}

#[test]
fn with_sort_direction_and_missing_criteria() {
    let sort = SortField::field("user.score")
        .in_descending_order()
        .where_missing_use(42)
        .build();

    assert_eq!(
        to_json(sort),
        json!({
            "user.score": {
                "order": "desc",
                "missing": 42
            }
        })
    );
}
