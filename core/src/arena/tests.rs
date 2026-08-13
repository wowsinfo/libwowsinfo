//! Tests against a real `tempArenaInfo.json` sample captured from the game.

use serde_json::Value;

use super::*;

/// Real arena file written by the game (`12x12`, two teams, 24 vehicles).
const ARENA_JSON: &str = r#"{
    "matchGroup": "pvp",
    "gameMode": 7,
    "clientVersionFromExe": "12,3,1,6965290",
    "mapDisplayName": "01_solomon_islands",
    "mapId": 1,
    "playersPerTeam": 12,
    "duration": 1200,
    "name": "12x12",
    "scenario": "Domination_3point",
    "playerID": 0,
    "vehicles": [
        {"shipId": 4290689008, "relation": 2, "id": 671300040, "name": "crazy_wanna"},
        {"shipId": 4186879312, "relation": 0, "id": 671163302, "name": "HenryQuan"},
        {"shipId": 4186847184, "relation": 1, "id": 268378400, "name": ":Tirpitz:"},
        {"shipId": 4266538992, "relation": 2, "id": 268378409, "name": ":Fletcher:"},
        {"shipId": 4266538992, "relation": 1, "id": 268378399, "name": ":Ghormley:"}
    ],
    "gameType": "RandomBattle",
    "dateTime": "08.05.2023 19:39:32",
    "mapName": "spaces/01_solomon_islands",
    "playerName": "HenryQuan",
    "scenarioConfigId": 14,
    "teamsCount": 2,
    "playerVehicle": "PVSC103-Vicente-Guerrero",
    "battleDuration": 1200
}"#;

fn arena() -> super::ArenaInfo {
    let json: Value = serde_json::from_str(ARENA_JSON).expect("valid fixture");
    parse_arena(&json).expect("arena parses")
}

#[test]
fn parses_arena_document() {
    let arena = arena();
    assert_eq!(arena.game_mode, 7);
    assert_eq!(arena.map_name, "spaces/01_solomon_islands");
    assert_eq!(arena.map_display_name, "01_solomon_islands");
    assert_eq!(arena.players_per_team, 12);
    assert_eq!(arena.teams_count, 2);
    assert_eq!(arena.duration, 1200);
    assert_eq!(arena.game_type, "RandomBattle");
    assert_eq!(arena.date_time, "08.05.2023 19:39:32");
    assert_eq!(arena.player_name, "HenryQuan");
    assert_eq!(arena.vehicles.len(), 5);
    assert_eq!(arena.vehicles[0].ship_id, 4_290_689_008);
    assert_eq!(arena.vehicles[1].name, "HenryQuan");
}

#[test]
fn rejects_empty_and_array_payloads() {
    assert!(parse_arena(&Value::Null).is_none());
    assert!(parse_arena(&serde_json::json!([])).is_none());
    assert!(parse_arena(&serde_json::json!({"vehicles": 1})).is_none());
}

#[test]
fn bot_names_are_detected() {
    assert!(is_bot(":Tirpitz:"));
    assert!(!is_bot("HenryQuan"));
    assert!(!is_bot("crazy_wanna"));
}

#[test]
fn teams_split_by_relation() {
    let arena = arena();
    let (ally, enemy) = teams(&arena);
    assert_eq!(ally.len(), 3, "HenryQuan + :Tirpitz: + :Ghormley:");
    assert_eq!(enemy.len(), 2, "crazy_wanna + :Fletcher:");
    assert_eq!(team_of(&arena.vehicles[0]), Team::Enemy);
    assert_eq!(team_of(&arena.vehicles[1]), Team::Ally);
}

#[test]
fn stat_lookups_skip_bots() {
    let arena = arena();
    let lookups = stat_lookups(&arena);
    assert_eq!(lookups.len(), 2);
    assert_eq!(lookups[0], ("crazy_wanna".to_string(), 4_290_689_008));
    assert_eq!(lookups[1], ("HenryQuan".to_string(), 4_186_879_312));
}

#[test]
fn empty_arena_yields_empty_teams() {
    let arena = super::ArenaInfo::default();
    assert!(teams(&arena).0.is_empty());
    assert!(teams(&arena).1.is_empty());
    assert!(stat_lookups(&arena).is_empty());
}
