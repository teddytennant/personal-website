//! Port of src/components/ui/AsciiArt.jsx.
//!
//! Only `AsciiPortrait` and `AsciiDivider` are reachable from the live routes;
//! AsciiFrame / AsciiMonogram / AsciiCompass are dead code in the React site and
//! are deliberately not ported.
//!
//! These signatures are a fixed contract — HomePage, ProjectsPage, BlogPage and
//! Navbar are all written against them.

use std::cell::RefCell;
use std::rc::Rc;

use leptos::html::Div;
use leptos::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use crate::motion::{map_range, prefers_reduced_motion, use_scroll_y, EASE_ENTER};

/// `useInView(ref, { once: true, margin })` — a bare in-view flag, with no style
/// coupling. motion.rs only exposes the reveal-flavoured version, so this is the
/// local equivalent: flips to `true` the first time the node intersects the
/// viewport (grown/shrunk by `root_margin`), then disconnects.
fn use_in_view(node: NodeRef<Div>, root_margin: &'static str) -> ReadSignal<bool> {
    let (in_view, set_in_view) = signal(false);

    Effect::new(move |prev: Option<bool>| {
        // Wire the observer up once, on the first render where the node exists.
        if prev == Some(true) {
            return true;
        }
        let Some(el) = node.get() else { return false };
        let el: web_sys::Element = el.unchecked_into();

        let observer: Rc<RefCell<Option<web_sys::IntersectionObserver>>> =
            Rc::new(RefCell::new(None));
        let obs_handle = observer.clone();

        let cb = Closure::wrap(Box::new(move |entries: js_sys::Array| {
            let entered = entries.iter().any(|e| {
                e.dyn_into::<web_sys::IntersectionObserverEntry>()
                    .map(|e| e.is_intersecting())
                    .unwrap_or(false)
            });
            if entered {
                set_in_view.set(true);
                if let Some(o) = obs_handle.borrow_mut().take() {
                    o.disconnect();
                }
            }
        }) as Box<dyn FnMut(js_sys::Array)>);

        let init = web_sys::IntersectionObserverInit::new();
        init.set_root_margin(root_margin);
        if let Ok(o) =
            web_sys::IntersectionObserver::new_with_options(cb.as_ref().unchecked_ref(), &init)
        {
            o.observe(&el);
            *observer.borrow_mut() = Some(o);
        }
        cb.forget();
        true
    });

    in_view
}

/* ═══════════════════════════════════════════════════════
   ASCII PORTRAIT — Hero profile signal
   A circular portrait framed like a transmission being
   locked on: rings settle, then the signal resolves.
   ═══════════════════════════════════════════════════════ */

/// Hero portrait: two signal rings settle, the image resolves out of a blur, a
/// scan sweep passes over it, and the whole thing parallaxes/fades as the hero
/// scrolls away (`scroll_ref` is the hero container it tracks).
#[component]
pub fn AsciiPortrait(
    scroll_ref: NodeRef<Div>,
    #[prop(into)] src: String,
    #[prop(into, default = String::from("Teddy Tennant"))] alt: String,
) -> impl IntoView {
    let reduced = prefers_reduced_motion();
    let node = NodeRef::<Div>::new();
    let is_in_view = use_in_view(node, "0px");

    // `useScroll({ target: scrollRef, offset: ['start start', 'end start'] })`:
    // 0 when the hero's top hits the viewport top, 1 when its bottom does.
    let scroll_y = use_scroll_y();
    let progress = Signal::derive(move || {
        // Depend on the scroll position so this recomputes as the page moves.
        let _ = scroll_y.get();
        let Some(el) = scroll_ref.get() else {
            return 0.0;
        };
        let el: web_sys::HtmlElement = el.unchecked_into();
        let height = el.offset_height() as f64;
        if height <= 0.0 {
            return 0.0;
        }
        let top = el.get_bounding_client_rect().top();
        (-top / height).clamp(0.0, 1.0)
    });

    // y:       [0, 1]      -> ['0%', '-20%']
    // opacity: [0, .5, 1]  -> [1, 0.5, 0]
    let outer_style = Signal::derive(move || {
        if reduced {
            return String::new();
        }
        let p = progress.get();
        let y = map_range(p, (0.0, 1.0), (0.0, -20.0));
        let opacity = if p <= 0.5 {
            map_range(p, (0.0, 0.5), (1.0, 0.5))
        } else {
            map_range(p, (0.5, 1.0), (0.5, 0.0))
        };
        format!("transform: translateY({y:.4}%); opacity: {opacity:.4};")
    });

    // Outer signal rings — opacity 0 -> 1, scale 0.9 -> 1 over 0.9s (ring two
    // with a 0.1s delay).
    let ring_style = move |delay: f64| {
        Signal::derive(move || {
            if reduced {
                return String::new();
            }
            let t = format!(
                "transition: opacity 0.9s {EASE_ENTER} {delay}s, transform 0.9s {EASE_ENTER} {delay}s;"
            );
            if is_in_view.get() {
                format!("opacity: 1; transform: scale(1); {t}")
            } else {
                format!("opacity: 0; transform: scale(0.9); {t}")
            }
        })
    };
    let ring_one_style = ring_style(0.0);
    let ring_two_style = ring_style(0.1);

    // Portrait — opacity 0 -> 1, scale 0.94 -> 1, blur(6px) -> blur(0px) over 1s.
    let portrait_style = Signal::derive(move || {
        if reduced {
            return String::new();
        }
        let t = format!(
            "transition: opacity 1s {EASE_ENTER}, transform 1s {EASE_ENTER}, filter 1s {EASE_ENTER};"
        );
        if is_in_view.get() {
            format!("opacity: 1; transform: scale(1); filter: blur(0px); {t}")
        } else {
            format!("opacity: 0; transform: scale(0.94); filter: blur(6px); {t}")
        }
    });

    // Scan sweep — travels top to bottom while fading out. Omitted under reduced motion.
    let sweep_style = Signal::derive(move || {
        let base = concat!(
            "background: linear-gradient(180deg, transparent 42%, var(--color-glow) 50%, transparent 58%); ",
            "mix-blend-mode: overlay; "
        );
        let t = format!(
            "transition: transform 1.1s {EASE_ENTER} 0.25s, opacity 1.1s {EASE_ENTER} 0.25s;"
        );
        if is_in_view.get() {
            format!("{base}transform: translateY(100%); opacity: 0; {t}")
        } else {
            format!("{base}transform: translateY(-100%); opacity: 0.7; {t}")
        }
    });

    view! {
        <div
            node_ref=node
            style=outer_style
            class="w-full h-full flex items-center justify-center"
        >
            <div class="relative">
                // Outer signal rings
                <div
                    class="absolute -inset-4 md:-inset-6 rounded-full border border-cream/10"
                    style=ring_one_style
                />
                <div
                    class="absolute -inset-8 md:-inset-11 rounded-full border border-cream/[0.06]"
                    style=ring_two_style
                />

                // Portrait
                <div
                    class="relative w-[180px] h-[180px] sm:w-[240px] sm:h-[240px] md:w-[320px] md:h-[320px] rounded-full overflow-hidden"
                    style=portrait_style
                >
                    <img
                        src=src
                        alt=alt
                        class="w-full h-full object-cover select-none"
                        draggable="false"
                    />

                    // Scan sweep — signal locking on
                    {(!reduced).then(|| {
                        view! {
                            <div class="absolute inset-0 pointer-events-none" style=sweep_style />
                        }
                    })}

                    // Ring highlight
                    <div class="absolute inset-0 rounded-full ring-1 ring-inset ring-cream/15 pointer-events-none" />
                </div>
            </div>
        </div>
    }
}

/* ═══════════════════════════════════════════════════════
   ASCII DIVIDER — Ornamental horizontal rule
   Animates outward from center character by character.
   ═══════════════════════════════════════════════════════ */

const DIVIDER_PATTERNS: [&str; 5] = [
    "════════════════════ ◆ ════════════════════",
    "──── · ──── · ──── ◆ ──── · ──── · ────",
    "═══╤═══╧═══╤═══ ◆ ═══╤═══╧═══╤═══",
    "· · · · · · · · · ◆ · · · · · · · · ·",
    "────────── ◇ ── ◆ ── ◇ ──────────",
];

/// Ornamental rule that animates outward from the centre, character by character.
#[component]
pub fn AsciiDivider(
    #[prop(default = 0)] seed: usize,
    #[prop(default = false)] dark: bool,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    let reduced = prefers_reduced_motion();
    let node = NodeRef::<Div>::new();
    let is_in_view = use_in_view(node, "-40px");

    let pattern = DIVIDER_PATTERNS[seed % DIVIDER_PATTERNS.len()];
    // JS `.split('')` is UTF-16 code units; every glyph here is BMP, so char
    // positions line up exactly with the React indices.
    let mid = pattern.chars().count() / 2;

    let outer_class = format!("py-6 overflow-hidden {class}");
    let inner_class = if dark {
        "font-mono text-[10px] md:text-[11px] text-center whitespace-pre select-none text-cream"
    } else {
        "font-mono text-[10px] md:text-[11px] text-center whitespace-pre select-none text-[#1a1a1a]"
    };

    let chars = pattern
        .chars()
        .enumerate()
        .map(|(i, ch)| {
            let is_glow = ch == '◆' || ch == '◇';
            let target = if is_glow {
                0.4
            } else if dark {
                0.15
            } else {
                0.12
            };
            let delay = (i as f64 - mid as f64).abs() * 0.012;
            let glow_class = if is_glow { "text-glow" } else { "" };

            let style = Signal::derive(move || {
                if reduced {
                    return format!("opacity: {target};");
                }
                let t = format!(
                    "transition: opacity 0.3s {EASE_ENTER} {delay:.3}s, transform 0.3s {EASE_ENTER} {delay:.3}s;"
                );
                if is_in_view.get() {
                    format!("opacity: {target}; transform: scale(1); {t}")
                } else {
                    format!("opacity: 0; transform: scale(0.5); {t}")
                }
            });

            view! { <span class=glow_class style=style>{ch.to_string()}</span> }
        })
        .collect_view();

    view! {
        <div node_ref=node class=outer_class aria-hidden="true">
            <div class=inner_class>{chars}</div>
        </div>
    }
}
