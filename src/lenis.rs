//! Rust replacement for `lenis` smooth scrolling.
//!
//! The React site mounts Lenis with `{ lerp: 0.07, duration: 1.6, smoothWheel: true }`.
//! When `lerp` is set, Lenis ignores `duration` and exponentially damps the animated
//! scroll position toward the wheel-accumulated target every frame.
//!
//! Lenis' per-frame factor of 0.07 at 60fps is reproduced here frame-rate independently:
//!   0.93 = e^(-k / 60)  =>  k = -60 * ln(0.93) ~= 4.345
//! so `animated += (target - animated) * (1 - e^(-k * dt))` matches Lenis at 60fps and
//! stays correct on 120Hz displays.

use std::cell::RefCell;
use std::rc::Rc;

use leptos::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

/// -60 * ln(1 - 0.07)
const DAMP_K: f64 = 4.3451;

/// Wheel deltas arrive in several units; normalise to pixels like Lenis does.
fn normalize_wheel_delta(e: &web_sys::WheelEvent) -> f64 {
    match e.delta_mode() {
        // DOM_DELTA_LINE
        1 => e.delta_y() * 16.0,
        // DOM_DELTA_PAGE
        2 => e.delta_y() * window().inner_height().ok().and_then(|v| v.as_f64()).unwrap_or(800.0),
        // DOM_DELTA_PIXEL
        _ => e.delta_y(),
    }
}

fn max_scroll() -> f64 {
    let doc = document();
    let body_h = doc
        .document_element()
        .map(|e| e.scroll_height() as f64)
        .unwrap_or(0.0);
    let view_h = window()
        .inner_height()
        .ok()
        .and_then(|v| v.as_f64())
        .unwrap_or(0.0);
    (body_h - view_h).max(0.0)
}

#[derive(Clone, Copy)]
pub struct Lenis {
    /// Set true to pause smooth scrolling (the mobile menu does this).
    pub stopped: RwSignal<bool>,
}

impl Lenis {
    pub fn stop(&self) {
        self.stopped.set(true);
    }
    pub fn start(&self) {
        self.stopped.set(false);
    }
    /// Jump to top instantly, cancelling any in-flight easing (route changes do this).
    pub fn scroll_to_top(&self) {
        RESET.with(|r| r.set(true));
        window().scroll_to_with_x_and_y(0.0, 0.0);
    }
}

thread_local! {
    static RESET: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
}

/// Mount the smooth scroller and provide a `Lenis` handle via context.
pub fn provide_lenis() -> Lenis {
    let stopped = RwSignal::new(false);
    let lenis = Lenis { stopped };
    provide_context(lenis);

    // Respect the user's motion preference — Lenis is pure eye-candy.
    if crate::motion::prefers_reduced_motion() {
        return lenis;
    }

    let target = Rc::new(RefCell::new(0.0_f64));
    let animated = Rc::new(RefCell::new(0.0_f64));
    let last_t = Rc::new(RefCell::new(f64::NAN));

    // `html.lenis` / `.lenis-smooth` are the hooks index.css already styles.
    if let Some(root) = document().document_element() {
        let _ = root.class_list().add_2("lenis", "lenis-smooth");
    }

    // ── wheel: accumulate into `target`, suppress native scrolling ──
    {
        let target = target.clone();
        let animated = animated.clone();
        let cb = Closure::<dyn FnMut(web_sys::WheelEvent)>::new(move |e: web_sys::WheelEvent| {
            if stopped.get_untracked() {
                return;
            }
            e.prevent_default();
            // Re-sync from the real scroll position if something else moved us.
            if RESET.with(|r| r.replace(false)) {
                let y = window().scroll_y().unwrap_or(0.0);
                *animated.borrow_mut() = y;
                *target.borrow_mut() = y;
            }
            let next = (*target.borrow() + normalize_wheel_delta(&e)).clamp(0.0, max_scroll());
            *target.borrow_mut() = next;
        });

        let opts = web_sys::AddEventListenerOptions::new();
        opts.set_passive(false);
        let _ = window().add_event_listener_with_callback_and_add_event_listener_options(
            "wheel",
            cb.as_ref().unchecked_ref(),
            &opts,
        );
        cb.forget();
    }

    // ── rAF: exponentially damp `animated` toward `target` ──
    {
        let target = target.clone();
        let animated = animated.clone();
        let last_t = last_t.clone();

        let f: Rc<RefCell<Option<Closure<dyn FnMut(f64)>>>> = Rc::new(RefCell::new(None));
        let g = f.clone();

        *g.borrow_mut() = Some(Closure::wrap(Box::new(move |now: f64| {
            let prev = *last_t.borrow();
            *last_t.borrow_mut() = now;
            let dt = if prev.is_nan() {
                1.0 / 60.0
            } else {
                ((now - prev) / 1000.0).clamp(0.0, 0.1)
            };

            if RESET.with(|r| r.replace(false)) {
                let y = window().scroll_y().unwrap_or(0.0);
                *animated.borrow_mut() = y;
                *target.borrow_mut() = y;
            }

            if !stopped.get_untracked() {
                let t = *target.borrow();
                let mut a = *animated.borrow();
                let diff = t - a;
                if diff.abs() > 0.08 {
                    a += diff * (1.0 - (-DAMP_K * dt).exp());
                    *animated.borrow_mut() = a;
                    window().scroll_to_with_x_and_y(0.0, a);
                } else if diff != 0.0 {
                    *animated.borrow_mut() = t;
                    window().scroll_to_with_x_and_y(0.0, t);
                }
            }

            let next = f.borrow();
            if let Some(cb) = next.as_ref() {
                let _ = window().request_animation_frame(cb.as_ref().unchecked_ref());
            }
        }) as Box<dyn FnMut(f64)>));

        let first = g.borrow();
        if let Some(cb) = first.as_ref() {
            let _ = window().request_animation_frame(cb.as_ref().unchecked_ref());
        }
    }

    lenis
}

pub fn use_lenis() -> Option<Lenis> {
    use_context::<Lenis>()
}
