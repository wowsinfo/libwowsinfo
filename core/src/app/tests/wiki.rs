//! Wiki / local-data flow tests.

use super::*;

#[test]
fn load_wiki_collections_populates_view() {
    let app = AppTester::<App>::default();
    let mut model = Model::default();
    let _ = app.update(Event::Init(config()), &mut model);
    let update = app.update(
        Event::LoadWiki {
            dataset: WikiDataset::Collections,
        },
        &mut model,
    );
    let event = resolve_http_matching(
        &app,
        update,
        "encyclopedia/collections",
        serde_json::json!({
            "status": "ok",
            "meta": {"page": 1, "page_total": 1},
            "data": {"1": {"collection_id": 1, "name": "C1", "description": "d", "image": "i"}}
        }),
    );
    let _ = app.update(event, &mut model);
    let collections = app.view(&model).wiki_collections;
    assert_eq!(collections.len(), 1);
    assert_eq!(collections.get(&1).map(|c| c.name.as_str()), Some("C1"));
    assert!(!model.downloading_wiki.contains(&WikiDataset::Collections));
}

#[test]

fn load_ship_wiki_populates_view() {
    let app = AppTester::<App>::default();
    let mut model = Model::default();
    let _ = app.update(Event::Init(config()), &mut model);
    let update = app.update(
        Event::LoadShipWiki {
            ship_id: 3542005744,
        },
        &mut model,
    );
    let event = resolve_http_matching(
        &app,
        update,
        "encyclopedia/ships",
        serde_json::json!({
            "status": "ok",
            "data": {"3542005744": {
                "ship_id": 3542005744u64,
                "name": "Hermelin",
                "type": "dd",
                "tier": 1,
                "default_profile": {
                    "armour": {"total": 51, "health": 37500},
                    "weaponry": {"artillery": 72},
                    "mobility": {"total": 55, "max_speed": 32.5}
                }
            }}
        }),
    );
    let _ = app.update(event, &mut model);
    let ship = app.view(&model).selected_ship_wiki.expect("ship wiki");
    assert_eq!(ship.name, "Hermelin");
    assert_eq!(ship.profile.armour.total, 51);
    assert_eq!(ship.profile.weaponry.artillery, 72);
}

#[test]

fn local_data_drives_ship_wiki_and_warship_list() {
    let app = AppTester::<App>::default();
    let mut model = Model::default();
    let _ = app.update(Event::Init(config()), &mut model);
    // KV is shared within the test process; pin the language explicitly.
    let _ = app.update(
        Event::SetLanguage {
            language: "en".to_string(),
        },
        &mut model,
    );
    let ships = serde_json::json!({
        "ships": {
            "1": {
                "id": 1, "index": "PASC014", "name": "IDS_NAME", "description": "IDS_D",
                "year": "IDS_Y", "paperShip": false, "tier": 8, "region": "USA",
                "type": "Cruiser", "regionID": "IDS_USA", "typeID": "IDS_CRUISER",
                "group": "normal", "costXP": 0, "costGold": 0, "costCR": 100,
                "consumables": [], "nextShips": [],
                "modules": {
                    "_Hull": [{
                        "index": 0, "name": "IDS_HULL", "cost": {"costXP": 0, "costCR": 0},
                        "components": {"hull": ["A_Hull"], "artillery": ["A1_Guns"]}
                    }]
                },
                "components": {
                    "A_Hull": {"health": 30000.0, "protection": 4.0,
                               "mobility": {"speed": 32.0},
                               "visibility": {"sea": 11.5}},
                    "A1_Guns": {"range": 14699.0, "sigma": 2.0, "guns": [{
                        "reload": 15.0, "rotation": 25.7, "each": 3,
                        "ammo": ["PAPA_AP"], "vertSector": 41.0, "count": 3}]}
                }
            }
        },
        "projectiles": {
            "PAPA_AP": {
                "type": "Artillery", "nation": "USA", "name": "IDS_AP",
                "ammoType": "AP", "speed": 800.0, "weight": 120.0, "damage": 5000.0,
                "diameter": 0.203,
                "ap": {"diameter": 0.203, "weight": 120.0, "drag": 0.3,
                       "velocity": 800.0, "krupp": 2400.0}
            }
        }
    })
    .to_string();
    let lang = serde_json::json!({
        "en": {
            "IDS_NAME": "New Orleans", "IDS_HULL": "Hull A", "IDS_AP": "203 mm AP",
            "IDS_CRUISER": "Cruiser", "IDS_USA": "U.S.A."
        },
        "ja": {
            "IDS_NAME": "ニューオーリンズ", "IDS_HULL": "船体A", "IDS_AP": "203mm AP",
            "IDS_CRUISER": "巡洋艦", "IDS_USA": "アメリカ"
        }
    })
    .to_string();

    let ships = zstd::stream::encode_all(ships.as_bytes(), 3).expect("compress ships");
    let lang = zstd::stream::encode_all(lang.as_bytes(), 3).expect("compress lang");
    let _ = app.update(Event::SetLocalData { ships, lang }, &mut model);
    assert!(app.view(&model).local_data_ready);
    let _ = app.update(Event::LoadLocalWarships, &mut model);
    assert_eq!(app.view(&model).warship.len(), 1);
    assert_eq!(app.view(&model).warship[&1].name, "New Orleans");

    let _ = app.update(
        Event::SetLanguage {
            language: "ja".to_string(),
        },
        &mut model,
    );
    assert_eq!(app.view(&model).warship[&1].name, "ニューオーリンズ");

    let _ = app.update(Event::LoadLocalShipWiki { ship_id: 1 }, &mut model);
    let wiki = app.view(&model).local_ship.expect("local ship");
    assert_eq!(wiki.name, "ニューオーリンズ");
    assert_eq!(wiki.main_battery.as_ref().map(|m| m.configuration.as_str()), Some("3 x 3"));
    assert_eq!(wiki.penetration_curves.len(), 1);

    let _ = app.update(
        Event::SelectLocalShipModule {
            slot: "artillery".to_string(),
            index: 0,
        },
        &mut model,
    );
    assert!(app.view(&model).local_ship.is_some());

    let _ = app.update(Event::LoadLocalCompare { ship_ids: vec![1] }, &mut model);
    let compare = app.view(&model).local_compare.expect("local compare");
    assert_eq!(compare.ships.len(), 1);
    assert_eq!(compare.rows.len(), 15);
    assert_eq!(compare.rows[0].label, "Tier");

    // Loadout toggles rebuild the local ship with adjusted stats.
    let _ = app.update(Event::SetLocalHp { fraction: 0.5 }, &mut model);
    let _ = app.update(Event::SetLocalSpotted { spotted: true }, &mut model);
    let _ = app.update(Event::ToggleLocalSkill { key: "x".to_string() }, &mut model);
    assert_eq!(app.view(&model).local_ship.as_ref().map(|s| s.hp_fraction), Some(0.5));
    assert!(app.view(&model).local_ship.as_ref().map(|s| s.spotted).unwrap_or(false));
}
