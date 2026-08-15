//! Wiki parser tests.

#[cfg(test)]
mod tests {
    use super::super::*;
    use serde_json::json;

    #[test]
    fn parses_battle_arenas() {
        let json = json!({
            "data": {
                "100": {
                    "arena_id": 100,
                    "name": "Islands of Ice",
                    "description": "A cold map.",
                    "icon": "https://example.com/map.jpg"
                },
                "200": {
                    "arena_id": 200,
                    "name": "Ocean",
                    "description": "Open water.",
                    "icon": ""
                },
                "300": {
                    "name": "Hotspot",
                    "description": "Tropical water.",
                    "icon": ""
                }
            }
        });
        let maps = parse_maps(&json);
        assert_eq!(maps.len(), 3);
        let map = &maps[&100];
        assert_eq!(map.name, "Islands of Ice");
        assert_eq!(map.description, "A cold map.");
        assert_eq!(map.icon, "https://example.com/map.jpg");
        assert_eq!(maps[&300].arena_id, 300);
        assert_eq!(maps[&300].name, "Hotspot");
    }
}
