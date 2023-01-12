use elastic_lens::request::search::{GeoPoint, SortGeo};
use serde_json::json;

fn to_json<S: serde::Serialize>(value: S) -> serde_json::Value {
    serde_json::to_value(value).unwrap()
}

const GEO_POINT: GeoPoint = GeoPoint { lat: 1.1, lon: 2.2 };

#[test]
fn simple_geo_distance_sort() {
    let sort = SortGeo::field_and_location("user.location", GEO_POINT).build();

    assert_eq!(
        to_json(sort),
        json!({
            "_geo_distance": {
                "user.location": { "lat": 1.1, "lon": 2.2 }
            }
        })
    );
}

#[test]
fn geo_distance_sort_with_arc_formula() {
    let sort = SortGeo::field_and_location("user.loc", GEO_POINT)
        .using_the_arc_formula()
        .build();

    assert_eq!(
        to_json(sort),
        json!({
            "_geo_distance": {
                "user.loc": { "lat": 1.1, "lon": 2.2 },
                "distance_type": "arc"
            }
        })
    );
}

#[test]
fn geo_distance_sort_with_plane_formula() {
    let sort = SortGeo::field_and_location("user.loc", GEO_POINT)
        .using_the_plane_formula()
        .build();

    assert_eq!(
        to_json(sort),
        json!({
            "_geo_distance": {
                "user.loc": { "lat": 1.1, "lon": 2.2 },
                "distance_type": "plane"
            }
        })
    );
}

#[test]
fn geo_distance_sort_ignoring_unmapped() {
    let sort = SortGeo::field_and_location("user.loc", GEO_POINT)
        .ignore_unmapped_documents()
        .build();

    assert_eq!(
        to_json(sort),
        json!({
            "_geo_distance": {
                "user.loc": { "lat": 1.1, "lon": 2.2 },
                "ignore_unmapped": true
            }
        })
    );
}

#[test]
fn geo_distance_sort_asc() {
    let sort = SortGeo::field_and_location("user.loc", GEO_POINT)
        .in_ascending_order()
        .build();

    assert_eq!(
        to_json(sort),
        json!({
            "_geo_distance": {
                "user.loc": { "lat": 1.1, "lon": 2.2 },
                "order": "asc"
            }
        })
    );
}

#[test]
fn geo_distance_sort_desc() {
    let sort = SortGeo::field_and_location("user.loc", GEO_POINT)
        .in_descending_order()
        .build();

    assert_eq!(
        to_json(sort),
        json!({
            "_geo_distance": {
                "user.loc": { "lat": 1.1, "lon": 2.2 },
                "order": "desc"
            }
        })
    );
}

#[test]
fn geo_distance_with_everything() {
    let sort = SortGeo::field_and_location("user.loc", GEO_POINT)
        .in_descending_order()
        .using_the_plane_formula()
        .ignore_unmapped_documents()
        .build();

    assert_eq!(
        to_json(sort),
        json!({
            "_geo_distance": {
                "user.loc": { "lat": 1.1, "lon": 2.2 },
                "order": "desc",
                "ignore_unmapped": true,
                "distance_type": "plane"
            }
        })
    );
}
