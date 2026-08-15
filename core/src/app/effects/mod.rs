//! Handlers for capability responses (HTTP, KV, time) and player assembly.
//! Split into domain submodules so every file stays small.

mod clan;
mod misc;
mod player;

pub(crate) use clan::{on_clan_info_loaded, on_clan_loaded, on_clan_search_loaded, on_clan_selected_loaded};
pub(crate) use misc::{on_game_version_loaded, on_kv_loaded, on_online_loaded, on_rank_loaded, on_rank_ships_loaded, on_ship_wiki_loaded, on_wiki_loaded};
pub(crate) use player::{on_achievements_loaded, on_achievements_wiki_loaded, on_player_loaded, on_pr_loaded, on_recent_loaded, on_search_loaded, on_ships_loaded, on_warship_loaded};

