use elastic_lens::request::search::{Distance, Field, GeoDistanceFilter, GeoPoint};

#[test]
fn building_a_geo_distance_filter() {
    let filter = GeoDistanceFilter::with_field("address.geo_point")
        .within(20)
        .miles()
        .of(GeoPoint::new(42.8, 13.6));

    assert_eq!(filter.field(), &Field::from("address.geo_point"));
    assert!(matches!(filter.distance(), Distance::Miles(20)));
    assert_eq!(filter.point(), GeoPoint::new(42.8, 13.6));
}
