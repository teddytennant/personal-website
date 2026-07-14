//! Port of src/context/ThemeContext.jsx.
//!
//! Toggles `html.light`, and only persists an *explicit* choice so first-time
//! visitors keep following their system preference until they touch the toggle.

use leptos::prelude::*;

const STORAGE_KEY: &str = "theme";

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Theme {
    Dark,
    Light,
}

impl Theme {
    pub fn as_str(&self) -> &'static str {
        match self {
            Theme::Dark => "dark",
            Theme::Light => "light",
        }
    }
}

#[derive(Clone, Copy)]
pub struct ThemeCtx {
    pub theme: RwSignal<Theme>,
}

impl ThemeCtx {
    pub fn set(&self, next: Theme) {
        self.theme.set(next);
        if let Ok(Some(store)) = window().local_storage() {
            let _ = store.set_item(STORAGE_KEY, next.as_str());
        }
    }
}

fn initial_theme() -> Theme {
    if let Ok(Some(store)) = window().local_storage() {
        match store.get_item(STORAGE_KEY) {
            Ok(Some(v)) if v == "light" => return Theme::Light,
            Ok(Some(v)) if v == "dark" => return Theme::Dark,
            _ => {}
        }
    }
    let prefers_light = window()
        .match_media("(prefers-color-scheme: light)")
        .ok()
        .flatten()
        .map(|m| m.matches())
        .unwrap_or(false);

    if prefers_light {
        Theme::Light
    } else {
        Theme::Dark
    }
}

pub fn provide_theme() -> ThemeCtx {
    let theme = RwSignal::new(initial_theme());
    let ctx = ThemeCtx { theme };
    provide_context(ctx);

    Effect::new(move |_| {
        let is_light = theme.get() == Theme::Light;
        if let Some(root) = document().document_element() {
            let list = root.class_list();
            if is_light {
                let _ = list.add_1("light");
            } else {
                let _ = list.remove_1("light");
            }
        }
    });

    ctx
}

pub fn use_theme() -> ThemeCtx {
    use_context::<ThemeCtx>().expect("ThemeCtx must be provided")
}
