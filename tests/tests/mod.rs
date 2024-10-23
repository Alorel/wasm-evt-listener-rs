mod util;

pub use util::*;

use futures::StreamExt;
use wasm_bindgen_test::wasm_bindgen_test;
use wasm_evt_listener::Listener;
use web_sys::CustomEvent;

#[wasm_bindgen_test]
pub async fn forward_clicks() {
    let target = mk_event_target();
    let listener = Listener::<CustomEvent>::builder()
        .with_passive(true)
        .build()
        .unwrap();
    listener.add_to("click", &target).unwrap();

    dispatch_event("click", 10, &target);
    dispatch_event("click", 56, &target);
    dispatch_event("click", 94, &target);

    let events = listener
        .take(3)
        .map(move |evt| evt.detail().as_f64().unwrap())
        .collect::<Vec<_>>()
        .await;

    assert_eq!(events, vec![10.0, 56.0, 94.0]);
}

#[wasm_bindgen_test]
pub async fn multi() {
    let target = mk_event_target();
    let listener = Listener::<CustomEvent>::builder()
        .build()
        .unwrap()
        .attach_multi([("foo".into(), &target), ("bar".into(), &target)])
        .unwrap();

    dispatch_event("foo", 100, &target);
    dispatch_event("bar", 200, &target);
    dispatch_event("qux", 220, &target);
    dispatch_event("foo", 50, &target);

    let events = listener
        .take(3)
        .map(move |evt| (evt.type_(), evt.detail().as_f64().unwrap()))
        .collect::<Vec<_>>()
        .await;

    assert_eq!(
        events,
        vec![
            (String::from("foo"), 100.0),
            ("bar".into(), 200.0),
            ("foo".into(), 50.0),
        ]
    );
}

#[wasm_bindgen_test]
pub async fn once() {
    let target = mk_event_target();
    let listener = Listener::<CustomEvent>::builder()
        .with_once(true)
        .build()
        .unwrap();

    let listener = listener
        .attach_multi([("a".into(), &target), ("b".into(), &target)])
        .unwrap();

    dispatch_event("a", 244, &target);
    dispatch_event("a", 222, &target);
    dispatch_event("b", 174, &target);

    let events = listener
        .take(2)
        .map(move |evt| evt.detail().as_f64().unwrap())
        .collect::<Vec<_>>()
        .await;

    assert_eq!(events, vec![244.0, 174.0]);
}
