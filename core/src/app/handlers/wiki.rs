//! Wiki (encyclopedia) request entry points.

use super::super::*;

pub(crate) fn wiki_url(
    dataset: WikiDataset,
    server: Server,
    key: &str,
    page: u64,
    language: &str,
) -> String {
    match dataset {
        WikiDataset::Collections => api::collections(server, key, page, language),
        WikiDataset::CollectionCards => api::collection_cards(server, key, page, language),
        WikiDataset::Consumables => api::consumables(server, key, page, language),
        WikiDataset::CommanderSkills => api::commander_skills(server, key, page, language),
        WikiDataset::Maps => api::battle_arenas(server, key, page, language),
    }
}


pub(crate) fn load_wiki(model: &mut Model, dataset: WikiDataset) -> Command<Effect, Event> {
    let loaded = match dataset {
        WikiDataset::Collections => !model.wiki_collections.is_empty(),
        WikiDataset::CollectionCards => !model.wiki_collection_cards.is_empty(),
        WikiDataset::Consumables => !model.wiki_consumables.is_empty(),
        WikiDataset::CommanderSkills => !model.wiki_commander_skills.is_empty(),
        WikiDataset::Maps => !model.wiki_maps.is_empty(),
    };
    if loaded || model.downloading_wiki.contains(&dataset) {
        return render::render();
    }
    let Some(config) = model.config.clone() else {
        return render::render();
    };
    let key = api_key(&config);
    if key.is_empty() {
        return render::render();
    }
    model.downloading_wiki.insert(dataset);
    HttpCap::get(wiki_url(dataset, model.server, &key, 1, &model.api_language))
        .expect_string()
        .build()
        .then_send(move |result| Event::WikiLoaded {
            dataset,
            outcome: http_outcome(result),
        })
}

/// Load the paginated ship encyclopedia, skipping when already loaded or a
/// download is in flight (used by the wiki ships tab).
pub(crate) fn load_warship(model: &mut Model) -> Command<Effect, Event> {
    if !model.warship.is_empty() || model.downloading_warship {
        return render::render();
    }
    let Some(config) = model.config.clone() else {
        return render::render();
    };
    let key = api_key(&config);
    if key.is_empty() {
        return render::render();
    }
    model.downloading_warship = true;
    HttpCap::get(api::warship(model.server, &key, 1, &model.api_language))
        .expect_string()
        .build()
        .then_send(|result| Event::WarshipLoaded(http_outcome(result)))
}

/// Load one ship's full wiki entry (`/wows/encyclopedia/ships/?ship_id=`).
pub(crate) fn load_ship_wiki(model: &mut Model, ship_id: u64) -> Command<Effect, Event> {
    let Some(config) = model.config.clone() else {
        return render::render();
    };
    let key = api_key(&config);
    if key.is_empty() {
        return render::render();
    }
    model.pending_ship_wiki_id = Some(ship_id);
    HttpCap::get(api::ship_wiki(model.server, &key, ship_id, &model.api_language))
        .expect_string()
        .build()
        .then_send(|result| Event::ShipWikiLoaded(http_outcome(result)))
}

