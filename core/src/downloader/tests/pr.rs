//! Personal-rating tests.



use super::super::*;


#[test]
fn pr_cleanup_drops_empty_arrays() {
    let json = serde_json::json!({
        "data": {
            "1": {"average_damage_dealt": 1.0, "average_frags": 0.5, "win_rate": 50.0},
            "2": []
        }
    });
    let parsed = parse_pr(&json);
    assert_eq!(parsed.len(), 1);
    assert!(parsed.contains_key(&1));
    assert!(!parsed.contains_key(&2));
}

#[test]
fn local_pr_has_data() {
    let pr = local_pr();
    assert!(
        pr.len() > 10,
        "bundled personal_rating.json should be usable"
    );
    let entry = pr.get(&3542005744).expect("known ship");
    assert!(entry.average_damage_dealt > 0.0);
}
