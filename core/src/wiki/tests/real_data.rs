//! Real-bundle parser tests (skipped in CI without the env vars).

use super::super::*;
#[test]
fn real_wowsinfo_json_parses_when_available() {
    // Smoke test against the shipped game data; skipped in CI without the env var.
    let Some(path) = std::env::var_os("WOWSINFO_JSON") else {
        return;
    };
    let raw = std::fs::read_to_string(path).expect("read wowsinfo.json");
    let json: serde_json::Value = serde_json::from_str(&raw).expect("valid json");
    let data = parse_game_data(&json);
    assert!(data.ships.len() > 1000, "ships: {}", data.ships.len());
    assert!(data.projectiles.len() > 2000, "projectiles: {}", data.projectiles.len());
    assert!(data.aircraft.len() > 500, "aircraft: {}", data.aircraft.len());
    assert!(data.modernizations.len() > 50, "modernizations: {}", data.modernizations.len());
    assert!(data.flags.len() >= 10, "flags: {}", data.flags.len());
    assert!(data.skills.len() > 50, "skills: {}", data.skills.len());
    assert!(data.abilities.len() > 100);
    assert!(data.achievements.len() > 100);

    let lang_path = std::env::var_os("WOWSINFO_LANG");
    if let Some(lang_path) = lang_path {
        let raw = std::fs::read_to_string(lang_path).expect("read lang.json");
        let json: serde_json::Value = serde_json::from_str(&raw).expect("valid lang json");
        let lang = parse_lang(&json, "en");
        assert!(lang.len() > 10_000, "lang entries: {}", lang.len());
    }

    // Every ship resolves to a local wiki entry with a hull.
    let sample = data.ships.iter().next().expect("at least one ship");
    let lang = LangMap::default();
    let wiki = build_local_ship_wiki(
        &data,
        &lang,
        *sample.0,
        ModuleSelection::default(),
        &LocalBuildConfig::default(),
    );
    assert!(wiki.is_some(), "ship {} builds", sample.1.index);

    // Compare + carrier views work against the real data.
    let compare = build_local_compare(&data, &lang, &[*sample.0]);
    assert!(compare.is_some());
    let carrier = data
        .ships
        .iter()
        .find(|(_, ship)| ship.r#type == "AirCarrier")
        .map(|(id, _)| *id);
    if let Some(carrier_id) = carrier {
        let wiki = build_local_ship_wiki(
            &data,
            &lang,
            carrier_id,
            ModuleSelection::default(),
            &LocalBuildConfig::default(),
        );
        assert!(wiki.is_some());
        assert!(!wiki.unwrap().aircraft.is_empty(), "carrier has squadrons");
    }
    // Lexington (PASA108) must expose fighter/torpedo/dive bomber slots.
    if let Some(ship) = data.ships.get(&4_181_702_640) {
        let wiki = build_local_ship_wiki(
            &data,
            &lang,
            4_181_702_640,
            ModuleSelection::default(),
            &LocalBuildConfig::default(),
        )
            .expect("lexington");
        assert!(
            wiki.aircraft.len() >= 3,
            "lexington aircraft slots: {:?}",
            wiki.aircraft.iter().map(|s| s.slot.clone()).collect::<Vec<_>>()
        );
        for slot in &wiki.aircraft {
            let resolved = slot
                .options
                .first()
                .and_then(|option| option.aircraft.as_ref());
            assert!(
                resolved.is_some(),
                "slot {} aircraft lookup failed",
                slot.slot
            );
            if let Some(plane) = resolved {
                assert!(
                    plane.attack_count.is_some_and(|count| count > 0),
                    "slot {} attack count populated",
                    slot.slot
                );
                assert!(
                    plane.attacker.is_some_and(|attacker| attacker > 0),
                    "slot {} attacker size populated",
                    slot.slot
                );
                assert!(plane.health > 0.0, "slot {} plane hp", slot.slot);
                assert!(plane.squadron_hp > 0.0, "slot {} squadron hp", slot.slot);
                assert!(
                    plane.attack_group_hp > 0.0,
                    "slot {} attack group hp",
                    slot.slot
                );
                if plane.aiming_accuracy_increase_rate.is_some() {
                    assert!(
                        plane.aiming_rate_moving_percent.is_some(),
                        "slot {} aiming rate moving",
                        slot.slot
                    );
                }
            }
        }
        let _ = ship;
    }
    // Loadouts resolve for a cruiser: consumables + skills + upgrades.
    if data.ships.contains_key(&4_293_834_736) {
        let wiki = build_local_ship_wiki(
            &data,
            &lang,
            4_293_834_736,
            ModuleSelection::default(),
            &LocalBuildConfig::default(),
        )
        .expect("cruiser");
        assert!(!wiki.skills.is_empty(), "cruiser has skills");
        assert!(
            !wiki.upgrades.is_empty(),
            "erie upgrades: {:?}",
            wiki.upgrades.iter().map(|u| u.key.clone()).collect::<Vec<_>>()
        );
        assert!(!wiki.flags.is_empty());
        assert!(!wiki.consumables.is_empty());
    }

    // A tier-10 battleship exposes all six upgrade slots, including the
    // concealment system (PCM027), per ShipBuilder's availability rules.
    if data.ships.contains_key(&3_760_142_320) {
        let wiki = build_local_ship_wiki(
            &data,
            &lang,
            3_760_142_320,
            ModuleSelection::default(),
            &LocalBuildConfig::default(),
        )
        .expect("montana");
        let slots: std::collections::HashSet<i64> =
            wiki.upgrades.iter().map(|u| u.slot).collect();
        assert!(
            slots.len() >= 6,
            "montana upgrade slots: {slots:?} ({} upgrades)",
            wiki.upgrades.len()
        );
        assert!(
            wiki.upgrades.iter().any(|u| u.key == "PCM027_ConcealmentMeasures_Mod_I"),
            "concealment upgrade missing: {:?}",
            wiki.upgrades.iter().map(|u| u.key.clone()).collect::<Vec<_>>()
        );
    }

    // Datong (PZSD728) exposes its regular shells plus the switchable-mode
    // alt shells: the alt HE has 30 mm penetration (vs 17 mm standard).
    if data.ships.contains_key(&3_531_486_416) {
        let wiki = build_local_ship_wiki(
            &data,
            &lang,
            3_531_486_416,
            ModuleSelection::default(),
            &LocalBuildConfig::default(),
        )
        .expect("datong");
        let shells = wiki
            .main_battery
            .as_ref()
            .map(|b| b.shells.clone())
            .unwrap_or_default();
        let he = shells
            .iter()
            .find(|s| s.ammo_type == "HE")
            .expect("datong HE shell");
        let ap = shells
            .iter()
            .find(|s| s.ammo_type == "AP")
            .expect("datong AP shell");
        assert_eq!(he.pen_he, Some(17.0));
        assert!(ap.damage > he.damage, "AP harder hitting than HE");
        let alt_he = shells
            .iter()
            .find(|s| s.ammo_type == "HE" && s.pen_he == Some(30.0))
            .expect("datong switchable HE with 30 mm pen");
        assert!(alt_he.key.contains("_ALT"));
        let burst = wiki
            .main_battery
            .as_ref()
            .and_then(|b| b.burst.as_ref())
            .expect("datong switchable mode");
        assert_eq!(burst.secondary_ammo.len(), 2);
    }

    // Zorkiy (PRSD111, T11) has a burst-fire switchable mode: 3 shots per
    // salvo, 1 s burst reload, 40 s full reload, +50% HE penetration.
    if data.ships.contains_key(&4_178_458_064) {
        let wiki = build_local_ship_wiki(
            &data,
            &lang,
            4_178_458_064,
            ModuleSelection::default(),
            &LocalBuildConfig::default(),
        )
        .expect("zorkiy");
        let burst = wiki
            .main_battery
            .as_ref()
            .and_then(|b| b.burst.as_ref())
            .expect("zorkiy burst mode");
        assert_eq!(burst.shots_count, 3);
        assert_eq!(burst.burst_reload_time, 1.0);
        assert_eq!(burst.full_reload_time, 40.0);
        assert_eq!(
            burst
                .modifiers
                .iter()
                .find(|(key, _)| key == "GMPenetrationCoeffHE"),
            Some(&("GMPenetrationCoeffHE".to_string(), 1.5))
        );
    }

    // A battleship resolves with a full armor digest (zones + turrets).
    let battleship = data
        .ships
        .iter()
        .find(|(_, ship)| ship.r#type == "Battleship")
        .map(|(id, _)| *id);
    if let Some(bb_id) = battleship {
        let wiki = build_local_ship_wiki(
            &data,
            &lang,
            bb_id,
            ModuleSelection::default(),
            &LocalBuildConfig::default(),
        );
        let wiki = wiki.expect("battleship wiki");
        let armor = wiki.armor.expect("battleship armor digest");
        assert!(armor.zone_count > 10, "zones: {}", armor.zone_count);
        assert!(!armor.turrets.is_empty(), "battleship has turrets");
        assert!(armor.turrets.iter().any(|turret| turret.armor > 0.0));
    }
}
