//! Port of src/hooks/useMediaQuery.js — a reactive `window.matchMedia`.

use leptos::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

pub fn use_media_query(query: &'static str) -> ReadSignal<bool> {
    let (matches, set_matches) = signal(false);

    Effect::new(move |prev: Option<bool>| {
        if prev == Some(true) {
            return true;
        }
        let Ok(Some(mql)) = window().match_media(query) else {
            return true;
        };
        set_matches.set(mql.matches());

        let mql2 = mql.clone();
        let cb = Closure::<dyn FnMut()>::new(move || set_matches.set(mql2.matches()));
        let _ = mql.add_event_listener_with_callback("change", cb.as_ref().unchecked_ref());
        cb.forget();
        true
    });

    matches
}
