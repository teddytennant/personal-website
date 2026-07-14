//! Port of src/App.jsx.

use leptos::prelude::*;
use leptos_router::components::Router;
use leptos_router::hooks::{use_location, use_navigate};
use leptos_router::NavigateOptions;

use crate::components::navbar::Navbar;
use crate::components::site_background::SiteBackground;
use crate::components::theme_toggle::ThemeToggle;
use crate::components::viewport_frame::ViewportFrame;
use crate::lenis::provide_lenis;
use crate::motion::{prefers_reduced_motion, EASE_ENTER};
use crate::pages::blog::BlogPage;
use crate::pages::home::HomePage;
use crate::pages::projects::ProjectsPage;
use crate::theme::provide_theme;

/// How long the outgoing page takes to leave, matching
/// `exit={{ opacity: 0, y: -20 }} transition={{ duration: 0.5 }}` under `mode="wait"`.
const EXIT_MS: u64 = 500;

#[derive(Clone, Copy, PartialEq)]
enum Phase {
    Entering,
    Leaving,
}

/// The three real routes. Everything else redirects, exactly as App.jsx does:
/// `/papers` and `/writing` -> `/blog`, and any other path -> `/`.
fn canonical(path: &str) -> &'static str {
    match path {
        "/" => "/",
        "/projects" => "/projects",
        "/blog" => "/blog",
        "/papers" | "/writing" => "/blog",
        _ => "/",
    }
}

/// React's `<AnimatePresence mode="wait">` holds the old page on screen while it
/// exits, then mounts the new one. Leptos' `<Routes>` swaps instantly, so the page
/// is selected here from a *delayed* copy of the location instead.
#[component]
fn AnimatedOutlet() -> impl IntoView {
    let location = use_location();
    let navigate = use_navigate();
    let reduced = prefers_reduced_motion();

    let shown = RwSignal::new(canonical(&location.pathname.get_untracked()));
    let phase = RwSignal::new(Phase::Entering);

    // Normalise the URL for the redirect paths so the address bar agrees with the view.
    Effect::new(move |_| {
        let raw = location.pathname.get();
        let target = canonical(&raw);
        if raw != target {
            navigate(target, NavigateOptions { replace: true, ..Default::default() });
        }
    });

    // Drive exit -> swap -> enter on every route change.
    Effect::new(move |prev: Option<&'static str>| {
        let next = canonical(&location.pathname.get());
        let Some(prev) = prev else { return next };
        if prev == next {
            return next;
        }

        if reduced {
            shown.set(next);
            window().scroll_to_with_x_and_y(0.0, 0.0);
            return next;
        }

        phase.set(Phase::Leaving);
        set_timeout(
            move || {
                shown.set(next);
                // ScrollToTop in App.jsx
                if let Some(l) = crate::lenis::use_lenis() {
                    l.scroll_to_top();
                } else {
                    window().scroll_to_with_x_and_y(0.0, 0.0);
                }
                // Start from the "below" position, then release on the next frame so
                // the browser actually transitions instead of snapping.
                request_animation_frame(move || phase.set(Phase::Entering));
            },
            std::time::Duration::from_millis(EXIT_MS),
        );
        next
    });

    let style = move || {
        if reduced {
            return String::new();
        }
        let t = format!(
            "transition: opacity 0.5s {EASE_ENTER}, transform 0.5s {EASE_ENTER}; will-change: opacity, transform;"
        );
        match phase.get() {
            Phase::Leaving => format!("opacity: 0; transform: translateY(-20px); {t}"),
            Phase::Entering => format!("opacity: 1; transform: translateY(0px); {t}"),
        }
    };

    view! {
        <main style=style>
            {move || match shown.get() {
                "/projects" => ProjectsPage().into_any(),
                "/blog" => BlogPage().into_any(),
                _ => HomePage().into_any(),
            }}
        </main>
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_theme();
    provide_lenis();

    view! {
        <Router>
            <div class="min-h-screen">
                <SiteBackground />
                <ViewportFrame />
                <Navbar />
                <ThemeToggle />
                <AnimatedOutlet />
            </div>
        </Router>
    }
}
