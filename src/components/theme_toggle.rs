//! Port of src/components/ui/ThemeToggle.jsx

use leptos::prelude::*;

use crate::theme::{use_theme, Theme};

#[component]
pub fn ThemeToggle() -> impl IntoView {
    let ctx = use_theme();

    let options = [(Theme::Dark, "Dark"), (Theme::Light, "Light")];

    view! {
        <div class="fixed z-50 flex items-center right-3 bottom-3 flex-row gap-3 rounded-full border border-cream/10 bg-ink/70 px-3.5 py-2 backdrop-blur-md md:right-5 md:bottom-auto md:top-1/2 md:-translate-y-1/2 md:flex-col md:gap-6 md:rounded-none md:border-0 md:bg-transparent md:px-0 md:py-0 md:backdrop-blur-none">
            {options
                .into_iter()
                .map(|(value, label)| {
                    let active = move || ctx.theme.get() == value;
                    view! {
                        <button
                            type="button"
                            on:click=move |_| ctx.set(value)
                            aria-pressed=move || active().to_string()
                            aria-label=format!("Switch to {} mode", value.as_str())
                            class=move || {
                                let base = "font-mono text-[10px] md:text-[9px] tracking-[0.25em] md:tracking-[0.3em] uppercase whitespace-nowrap transition-colors md:[writing-mode:vertical-rl] md:rotate-180";
                                if active() {
                                    format!("{base} text-cream")
                                } else {
                                    format!("{base} text-muted/60 hover:text-cream")
                                }
                            }
                        >
                            <span aria-hidden="true">{move || if active() { "■" } else { "□" }}</span>
                            " "
                            {label}
                        </button>
                    }
                })
                .collect_view()}
        </div>
    }
}
