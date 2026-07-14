//! Rust replacement for the site's `motion/react` (Framer Motion) usage.
//!
//! The React site only ever asks Framer Motion for three things:
//!   1. scroll-triggered reveals (`whileInView` + `once`)  -> IntersectionObserver
//!   2. mount-time enter animations with a stagger delay   -> a delayed class flip
//!   3. one scroll-linked opacity fade (the scroll cue)    -> a scroll listener
//!
//! All three are driven here by CSS transitions on inline styles, which is what
//! Framer Motion itself compiles down to, so the visual result is the same.

use leptos::html::ElementType;
use leptos::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

/// `EASE_ENTER` from src/utils/motion.js — [0.16, 1, 0.3, 1]
pub const EASE_ENTER: &str = "cubic-bezier(0.16, 1, 0.3, 1)";

/// Mirrors `useReducedMotion()`.
pub fn prefers_reduced_motion() -> bool {
    window()
        .match_media("(prefers-reduced-motion: reduce)")
        .ok()
        .flatten()
        .map(|m| m.matches())
        .unwrap_or(false)
}

fn transition(dur: f64) -> String {
    format!(
        "transition: opacity {dur}s {EASE_ENTER}, transform {dur}s {EASE_ENTER}; will-change: opacity, transform;"
    )
}

/// Inline style for an element that animates from `y`px + transparent to rest.
fn style_for(visible: bool, y: f64, dur: f64) -> String {
    if visible {
        format!("opacity: 1; transform: translateY(0px); {}", transition(dur))
    } else {
        format!(
            "opacity: 0; transform: translateY({y}px); {}",
            transition(dur)
        )
    }
}

/// Attach an IntersectionObserver to `node` that fires once, then disconnects.
fn observe_once<E>(node: NodeRef<E>, root_margin: &'static str, on_enter: impl Fn() + 'static)
where
    E: ElementType + 'static,
    E::Output: JsCast + Clone + 'static,
{
    // The Effect body is `FnMut` (it may re-run), so the callback has to be shared
    // rather than moved out of the environment.
    let on_enter = std::rc::Rc::new(on_enter);

    Effect::new(move |prev: Option<bool>| {
        // Only wire the observer up once, on the first render where the node exists.
        if prev == Some(true) {
            return true;
        }
        let Some(el) = node.get() else { return false };
        let el: web_sys::Element = el.unchecked_into();

        let observer: std::rc::Rc<std::cell::RefCell<Option<web_sys::IntersectionObserver>>> =
            std::rc::Rc::new(std::cell::RefCell::new(None));
        let obs_handle = observer.clone();
        let on_enter = on_enter.clone();

        let cb = Closure::wrap(Box::new(move |entries: js_sys::Array| {
            let entered = entries.iter().any(|e| {
                e.dyn_into::<web_sys::IntersectionObserverEntry>()
                    .map(|e| e.is_intersecting())
                    .unwrap_or(false)
            });
            if entered {
                on_enter();
                if let Some(o) = obs_handle.borrow_mut().take() {
                    o.disconnect();
                }
            }
        }) as Box<dyn FnMut(js_sys::Array)>);

        let init = web_sys::IntersectionObserverInit::new();
        init.set_root_margin(root_margin);
        if let Ok(o) = web_sys::IntersectionObserver::new_with_options(
            cb.as_ref().unchecked_ref(),
            &init,
        ) {
            o.observe(&el);
            *observer.borrow_mut() = Some(o);
        }
        cb.forget();
        true
    });
}

/// `whileInView` reveal: `{ opacity: 0, y: 18 } -> { opacity: 1, y: 0 }` over 0.7s,
/// triggered once when the element crosses `viewport.margin: '0px 0px -15% 0px'`.
///
/// Matches the `cell` variant + `revealProps` in HomePage.jsx.
pub fn use_reveal<E>(node: NodeRef<E>) -> Signal<String>
where
    E: ElementType + 'static,
    E::Output: JsCast + Clone + 'static,
{
    let reduced = prefers_reduced_motion();
    let visible = RwSignal::new(reduced);

    if !reduced {
        observe_once(node, "0px 0px -15% 0px", move || visible.set(true));
    }

    Signal::derive(move || {
        if reduced {
            String::new()
        } else {
            style_for(visible.get(), 18.0, 0.7)
        }
    })
}

/// Mount-time enter animation with a stagger delay, for the hero.
/// Matches `container` (staggerChildren 0.07, delayChildren 0.15) + `cell` in HomePage.jsx.
pub fn use_enter(index: usize) -> Signal<String> {
    let reduced = prefers_reduced_motion();
    let visible = RwSignal::new(reduced);

    if !reduced {
        // delayChildren 0.15 + staggerChildren 0.07 * index
        let delay_ms = (150.0 + 70.0 * index as f64) as i32;
        request_animation_frame(move || {
            set_timeout(move || visible.set(true), std::time::Duration::from_millis(delay_ms as u64));
        });
    }

    Signal::derive(move || {
        if reduced {
            String::new()
        } else {
            style_for(visible.get(), 18.0, 0.7)
        }
    })
}

/// Live `window.scrollY`, for scroll-linked transforms.
pub fn use_scroll_y() -> ReadSignal<f64> {
    let (scroll_y, set_scroll_y) = signal(0.0);

    Effect::new(move |prev: Option<bool>| {
        if prev == Some(true) {
            return true;
        }
        let cb = Closure::<dyn FnMut()>::new(move || {
            set_scroll_y.set(window().scroll_y().unwrap_or(0.0));
        });
        let _ = window().add_event_listener_with_callback("scroll", cb.as_ref().unchecked_ref());
        cb.forget();
        true
    });

    scroll_y
}

/// `useTransform(scrollY, [0, 120], [1, 0])` — the scroll cue's fade-out.
pub fn map_range(v: f64, from: (f64, f64), to: (f64, f64)) -> f64 {
    if (from.1 - from.0).abs() < f64::EPSILON {
        return to.0;
    }
    let t = ((v - from.0) / (from.1 - from.0)).clamp(0.0, 1.0);
    to.0 + t * (to.1 - to.0)
}
