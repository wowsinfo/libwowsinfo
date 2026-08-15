//! Clan flow tests.

use super::*;

#[test]
fn select_clan_loads_clan_info_into_view() {
    let app = AppTester::<App>::default();
    let mut model = Model::default();
    let _ = app.update(Event::Init(config()), &mut model);
    let update = app.update(Event::SelectClan { clan_id: 99 }, &mut model);
    let event = resolve_http_matching(
        &app,
        update,
        "clans/info",
        serde_json::json!({"status": "ok", "data": {"99": {
            "clan_id": 99,
            "tag": "ABC",
            "name": "Alpha",
            "members_count": 1,
            "members": {"7": {"account_name": "Bob", "role": "commander", "joined_at": 100}}
        }}}),
    );
    let _ = app.update(event, &mut model);
    let clan = app.view(&model).selected_clan.expect("selected clan");
    assert_eq!(clan.name, "Alpha");
    assert_eq!(clan.members.len(), 1);
    assert_eq!(clan.members[0].account_name, "Bob");
}
