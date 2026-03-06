use elastic_lens::request::search::{ScalarValue, ScriptFilterBuilder};
use serde_json::{json, to_value};

#[test]
fn builder_usage() {
    let filter = ScriptFilterBuilder::new("doc['field'].value > 10")
        .with_params([
            ("param1", ScalarValue::from("value1")),
            ("param2", ScalarValue::from(42)),
        ])
        .build();

    assert_eq!(
        to_value(filter).unwrap(),
        json!({
            "script": {
                "script": {
                    "source": "doc['field'].value > 10",
                    "params": {
                        "param1": "value1",
                        "param2": 42
                    }
                }
            }
        })
    );
}
