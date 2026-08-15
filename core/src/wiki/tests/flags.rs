//! Signal-flag real-data tests.


use super::super::*;

#[test]
fn flags_expose_wiki_entries() {
    let Some(path) = std::env::var_os("WOWSINFO_JSON") else {
        return;
    };
    let raw = std::fs::read_to_string(path).expect("read wowsinfo.json");
    let json: serde_json::Value = serde_json::from_str(&raw).expect("valid json");
    let data = parse_game_data(&json);
    let flags = all_flag_views(&data, &LangMap::default());
    assert_eq!(flags.len(), 15, "signal flags: {}", flags.len());
    assert!(
        flags.iter().any(|flag| flag.key == "PCEF005_SM_SignalFlag"),
        "sample flag present"
    );
    assert!(
        flags
            .iter()
            .find(|flag| flag.key == "PCEF005_SM_SignalFlag")
            .is_some_and(|flag| !flag.summary.is_empty()),
        "sample flag carries a modifier summary"
    );
}
