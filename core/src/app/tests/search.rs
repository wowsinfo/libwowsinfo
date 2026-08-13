#![allow(deprecated)]

use super::*;

#[test]
fn search_flow_returns_results() {
    let app = AppTester::<App>::default();
    let mut model = Model::default();
    let _ = app.update(Event::Init(config()), &mut model);

    let update = app.update(
        Event::SearchPlayer {
            query: "henry".to_string(),
        },
        &mut model,
    );
    assert!(matches!(model.phase, Phase::Searching));

    let body = serde_json::json!({
        "status": "ok",
        "data": [{"account_id": 1, "nickname": "HenryQuan"}]
    });
    let event = resolve_http_matching(&app, update, "/wows/account/list/", body);
    let Event::SearchLoaded(outcome) = event else {
        panic!("expected SearchLoaded, got {event:?}");
    };
    let _ = app.update(Event::SearchLoaded(outcome), &mut model);

    assert!(matches!(model.phase, Phase::Idle));
    assert_eq!(model.search_results.len(), 1);
    assert_eq!(app.view(&model).search_results[0].nickname, "HenryQuan");
}
#[test]
fn search_error_sets_phase() {
    let app = AppTester::<App>::default();
    let mut model = Model::default();
    let _ = app.update(Event::Init(config()), &mut model);
    let update = app.update(
        Event::SearchPlayer {
            query: "nobody".to_string(),
        },
        &mut model,
    );
    let mut update = update;
    let effect = update
        .effects_mut()
        .find(|e| matches!(e, Effect::Http(_)))
        .expect("http effect");
    let Effect::Http(request) = effect else {
        unreachable!()
    };
    let update = app
        .resolve(
            request,
            HttpResult::Err(crux_http::HttpError::from(std::io::Error::new(
                std::io::ErrorKind::Other,
                "network down",
            ))),
        )
        .expect("resolve");
    let event = update.events.into_iter().next().expect("event");
    let _ = app.update(event, &mut model);
    assert!(matches!(model.phase, Phase::Error(_)));
}
