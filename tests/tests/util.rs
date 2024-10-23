use wasm_bindgen::prelude::*;
use web_sys::{CustomEvent, CustomEventInit, EventTarget};

pub fn mk_event_target() -> EventTarget {
    js_sys::global().unchecked_into()
}

pub fn create_event(name: &str, detail: u8) -> CustomEvent {
    CustomEvent::new_with_event_init_dict(name, &{
        let init = CustomEventInit::new();
        init.set_detail(&detail.into());
        init
    })
    .expect("create_event()")
}

pub fn dispatch_event(name: &str, detail: u8, target: &EventTarget) -> CustomEvent {
    let event = create_event(name, detail);
    target.dispatch_event(&event).expect("dispatch_event()");
    event
}
