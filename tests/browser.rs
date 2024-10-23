wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

use futures::StreamExt;
use tests::dispatch_event;
use wasm_bindgen::prelude::*;
use wasm_bindgen_test::wasm_bindgen_test;
use wasm_evt_listener::Listener;
use web_sys::{CustomEvent, EventTarget, Window};

pub mod tests;

#[wasm_bindgen_test]
pub async fn multi_target() {
    let tgt1 = mk_event_target();
    let tgt2 = mk_event_target();

    let listener = Listener::<CustomEvent>::builder()
        .build()
        .unwrap()
        .attach_multi([
            ("Loch Lomond".into(), &tgt1),
            ("Loch Ness".into(), &tgt2),
            ("foo".into(), &tgt1),
            ("foo".into(), &tgt2),
        ])
        .unwrap();

    dispatch_event("foo", 50, &tgt1);
    dispatch_event("foo", 55, &tgt2);
    dispatch_event("Loch Lomond", 7, &tgt1);
    dispatch_event("Loch Ness", 9, &tgt2);
    dispatch_event("foo", 55, &tgt2);

    let events = listener
        .take(5)
        .map(move |evt| (evt.type_(), evt.detail().as_f64().unwrap()))
        .collect::<Vec<_>>()
        .await;

    assert_eq!(
        events,
        vec![
            (String::from("foo"), 50.0),
            ("foo".into(), 55.0),
            ("Loch Lomond".into(), 7.0),
            ("Loch Ness".into(), 9.0),
            ("foo".into(), 55.0),
        ]
    );
}

fn mk_event_target() -> EventTarget {
    js_sys::global()
        .unchecked_into::<Window>()
        .document()
        .expect("document()")
        .create_element("span")
        .expect("create_element()")
        .unchecked_into()
}
