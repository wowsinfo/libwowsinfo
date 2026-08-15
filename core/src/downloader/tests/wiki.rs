//! Wiki parser tests.



use super::super::*;


#[test]
fn wiki_pages_parse_and_key_by_id() {
    let collections = serde_json::json!({
        "status": "ok",
        "data": {"1": {"collection_id": 1, "name": "C1", "description": "d", "image": "i"}}
    });
    let cards = serde_json::json!({
        "status": "ok",
        "data": {"7": {
            "card_id": 7,
            "collection_id": 1,
            "name": "card",
            "description": "d",
            "images": {"small": "s"}
        }}
    });
    let consumables = serde_json::json!({
        "status": "ok",
        "data": {"3": {
            "consumable_id": 3,
            "name": "Repair",
            "description": "fix",
            "image": "i",
            "type": "repair",
            "price_credit": 100,
            "price_gold": 0,
            "profile": {"p1": {"description": "heals"}}
        }}
    });
    let skills = serde_json::json!({
        "status": "ok",
        "data": {"5": {
            "name": "Expert",
            "icon": "ic",
            "type_id": 1,
            "type_name": "skill",
            "customization": {
                "Destroyer": {"tier": 3, "column": 1, "perks": [{"perk_id": 9, "description": "bonus"}]},
                "Cruiser": {"tier": 4, "column": 0, "perks": [{"perk_id": 9, "description": "bonus"}, {"perk_id": 10, "description": "extra"}]}
            }
        }}
    });
    let cols = parse_collections(&collections);
    assert_eq!(cols.get(&1).map(|c| c.name.as_str()), Some("C1"));
    let card_list = parse_collection_cards(&cards);
    assert_eq!(card_list.get(&7).map(|c| c.collection_id), Some(1));
    assert_eq!(card_list.get(&7).map(|c| c.image.as_str()), Some("s"));
    let cons = parse_consumables(&consumables);
    let c = cons.get(&3).expect("consumable");
    assert_eq!(c.name, "Repair");
    assert_eq!(c.price_credit, 100);
    assert_eq!(c.profile.len(), 1);
    assert_eq!(c.profile[0].description, "heals");
    let sk = parse_commander_skills(&skills);
    let s = sk.get(&5).expect("skill");
    assert_eq!(s.skill_id, 5);
    assert_eq!(s.tier, 4, "highest class tier");
    assert_eq!(s.perks.len(), 2, "perks deduped across classes");
    assert_eq!(s.perks[0].perk_id, 9);
    assert_eq!(s.description, "bonus\nextra");
}

#[test]
fn ship_wiki_parses_profile_and_artillery() {
    let json = serde_json::json!({
        "status": "ok",
        "data": {"3542005744": {
            "ship_id": 3542005744u64,
            "name": "Hermelin",
            "description": "d",
            "nation": "pan_europe",
            "type": "dd",
            "tier": 1,
            "is_premium": false,
            "price_credit": 100,
            "price_gold": 0,
            "next_ships": [2],
            "images": {"small": "i"},
            "default_profile": {
                "armour": {"total": 51, "health": 37500, "citadel": {"min": -1, "max": -1}, "deck": {"min": 16, "max": 16}, "casemate": {"min": -1, "max": -1}, "extremities": {"min": -1, "max": -1}},
                "mobility": {"total": 55, "max_speed": 32.5, "turning_radius": 660, "rudder_time": 7.2},
                "concealment": {"total": 61, "detect_distance_by_ship": 11.5, "detect_distance_by_plane": 7.2, "detect_distance_by_submarine": 7.2},
                "weaponry": {"artillery": 72, "torpedoes": 0, "anti_aircraft": 77, "aircraft": 0},
                "artillery": {
                    "slots": {"0": {"name": "152 mm/47 Mk.16", "barrels": 3, "guns": 4}},
                    "shells": {"AP": {"name": "152 mm AP", "type": "AP", "damage": 3200, "bullet_mass": 59, "bullet_speed": 762}},
                    "gun_rate": 8.6, "max_dispersion": 140, "distance": 15.6
                },
                "torpedoes": null,
                "anti_aircraft": {"defense": 77, "slots": {"0": {"name": "20 mm Oerlikon", "caliber": 20, "guns": 23}}},
                "hull": {"health": 37500, "artillery_barrels": 4, "torpedoes_barrels": 0, "anti_aircraft_barrels": 39},
                "engine": {"max_speed": 32.5}
            }
        }}
    });
    let ship = parse_ship_wiki(&json, 3542005744).expect("ship");
    assert_eq!(ship.name, "Hermelin");
    assert_eq!(ship.profile.armour.total, 51);
    assert_eq!(ship.profile.armour.deck.max, 16);
    assert_eq!(ship.profile.weaponry.artillery, 72);
    let art = ship.profile.artillery.expect("artillery");
    assert_eq!(art.shells.len(), 1);
    assert_eq!(art.shells[0].r#type, "AP");
    assert_eq!(art.shells[0].bullet_speed, 762.0);
    assert_eq!(ship.profile.mobility.max_speed, 32.5);
    assert_eq!(ship.profile.anti_aircraft.as_ref().map(|a| a.defense), Some(77));
    assert_eq!(ship.next_ships, vec![2]);
    assert!(parse_ship_wiki(&json, 1).is_none());
}
