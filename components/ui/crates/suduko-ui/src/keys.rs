//! Browser keyboard wiring: parses keydown events into game keys and
//! installs one document-level handler.

use wasm_bindgen::prelude::*;
use yew::Callback;

pub(crate) enum Key {
    Digit(u8),
    Space,
    Escape,
}

pub(crate) fn parse_key(code: &str, key: &str) -> Option<Key> {
    match code {
        "Space" => Some(Key::Space),
        "Escape" => Some(Key::Escape),
        _ => {
            let bytes = key.as_bytes();
            if bytes.len() == 1 && (b'1'..=b'9').contains(&bytes[0]) {
                Some(Key::Digit(bytes[0] - b'0'))
            } else {
                None
            }
        }
    }
}

pub(crate) fn install_key_handler(send: Callback<Key>) {
    let handler =
        Closure::<dyn Fn(web_sys::KeyboardEvent)>::new(move |e: web_sys::KeyboardEvent| {
            if let Some(k) = parse_key(&e.code(), &e.key()) {
                e.prevent_default();
                send.emit(k);
            }
        });
    web_sys::window()
        .expect("window exists")
        .add_event_listener_with_callback(
            "keydown",
            handler.as_ref().unchecked_ref::<js_sys::Function>(),
        )
        .expect("listener installs");
    handler.forget();
}
