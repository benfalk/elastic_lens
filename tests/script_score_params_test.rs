use elastic_lens::request::search::ScriptParams;
use serde_json::{json, to_value};

#[test]
fn script_params_usage() {
    let mut params = ScriptParams::with_capacity(4);
    params.insert_scalar("lucky", 42);
    params.insert_scalar("abort", true);
    params.insert_array("values", [2, 4, 8]);

    assert_eq!(
        to_value(params).unwrap(),
        json!({
            "lucky": 42,
            "abort": true,
            "values": [2, 4, 8]
        })
    );
}
