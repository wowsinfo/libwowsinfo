//! Port of `src/core/downloader/Downloader.ts` data processing, reduced to
//! pure functions (side effects are requested by the Crux app instead).
//! Split into domain submodules so every file stays small.

mod assemble;
mod clan;
mod dates;
mod guard;
mod player;
mod pr;
mod rank;
mod recent;
mod search;
mod stats;
mod version;
mod wiki;

pub use assemble::assemble_player;
pub use clan::{parse_clan_id, parse_clan_info, parse_clan_search};
pub use dates::recent_dates;
pub use guard::{clean_pr_data, guard};
pub use player::{parse_achievements, parse_achievements_wiki, parse_clan_tag, parse_online_count, parse_player_info};
pub use pr::{local_pr, parse_pr};
pub use rank::{parse_rank_info, parse_rank_ship_stats};
pub use recent::parse_recent_overview;
pub use search::{parse_search_results, process_warship_entry};
pub use stats::parse_ship_stats;
pub use version::{check_version_update, is_ok};
pub use wiki::{
    parse_collection_cards, parse_collections, parse_commander_skills, parse_consumables,
    parse_maps, parse_ship_wiki,
};

#[cfg(test)]
mod tests;

