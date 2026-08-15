//! Search tests.



use super::super::*;


#[test]
fn search_results_parse() {
    let json = serde_json::json!({
        "status": "ok",
        "data": [{"account_id": 123, "nickname": "HenryQuan"}, {"account_id": 456, "nickname": "x"}]
    });
    let results = parse_search_results(&json);
    assert_eq!(results.len(), 2);
    assert_eq!(results[0].nickname, "HenryQuan");
}
