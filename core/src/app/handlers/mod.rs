//! Event entry points: initialisation, server selection, search and refresh.
//! Split into domain submodules so every file stays small.

mod local;
mod player;
mod wiki;

pub(crate) use local::{
    load_local_compare, load_local_ship_wiki, load_local_warships, select_local_ship_module,
    set_language, set_local_data, set_local_hp, set_local_spotted, toggle_local_flag,
    toggle_local_skill, toggle_local_upgrade,
};
pub(crate) use player::{init, refresh, search, search_clan, select, select_clan, set_server};
pub(crate) use wiki::{load_ship_wiki, load_warship, load_wiki};
pub(crate) use wiki::wiki_url;

