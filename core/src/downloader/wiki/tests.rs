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
                }
            }
        });
        let maps = parse_maps(&json);
        assert_eq!(maps.len(), 2);
        let map = &maps[&100];
        assert_eq!(map.name, "Islands of Ice");
        assert_eq!(map.description, "A cold map.");
        assert_eq!(map.icon, "https://example.com/map.jpg");
    }
}
