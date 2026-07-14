//! Port of src/components/layout/ViewportFrame.jsx

use leptos::prelude::*;

#[component]
pub fn ViewportFrame() -> impl IntoView {
    view! {
        <div
            class="fixed inset-2 md:inset-3 z-40 pointer-events-none border border-white/40 mix-blend-difference"
            aria-hidden="true"
        >
            <span class="absolute -top-px -left-px w-3 h-3 border-t border-l border-white/70" />
            <span class="absolute -top-px -right-px w-3 h-3 border-t border-r border-white/70" />
            <span class="absolute -bottom-px -left-px w-3 h-3 border-b border-l border-white/70" />
            <span class="absolute -bottom-px -right-px w-3 h-3 border-b border-r border-white/70" />
        </div>
    }
}
