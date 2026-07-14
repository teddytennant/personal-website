//! Port of src/components/layout/SiteBackground.jsx
//!
//! The GrainGradient itself has two interchangeable implementations behind a
//! cargo feature; this component only owns the palette and the overlay gradient.
use leptos::prelude::*;

use crate::bg::GrainGradient;
use crate::theme::{use_theme, Theme};

/// PALETTES from SiteBackground.jsx
pub const DARK_BACK: &str = "#050504";
pub const DARK_COLORS: [&str; 4] = ["#141412", "#3A3A34", "#6B6A62", "#B8B6AC"];
pub const LIGHT_BACK: &str = "#F6F5F1";
pub const LIGHT_COLORS: [&str; 4] = ["#EFEDE7", "#E0DDD2", "#CCC8BB", "#A5A296"];

#[component]
pub fn SiteBackground() -> impl IntoView {
    let ctx = use_theme();
    let palette = move || match ctx.theme.get() {
        Theme::Light => (LIGHT_BACK, LIGHT_COLORS),
        Theme::Dark => (DARK_BACK, DARK_COLORS),
    };

    view! {
        <div
            class="fixed inset-0 -z-10 overflow-hidden bg-ink pointer-events-none"
            aria-hidden="true"
        >
            <GrainGradient palette=Signal::derive(palette) />
            <div class="absolute inset-0 bg-gradient-to-b from-ink/55 via-ink/30 to-ink/70" />
        </div>
    }
}
