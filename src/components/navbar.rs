//! Port of src/components/layout/Navbar.jsx.
//!
//! Framer Motion is replaced with CSS transitions on inline styles (the same thing
//! Framer compiles down to). Three animations live here:
//!
//!   1. the nav's mount fade   (opacity 0 -> 1, 0.8s, delay 0.2s)
//!   2. the wordmark swap      (`AnimatePresence mode="wait"`, 0.25s out then 0.25s in)
//!   3. the mobile menu        (container fade + staggered items)
//!
//! `lucide-react`'s Menu / X / Github glyphs are inlined as SVG rather than pulling in
//! a Rust icon crate. The JSX also defines an `XIcon` (the X/Twitter brand mark) but
//! never renders it, so it is not ported.

use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;

use leptos::prelude::*;
use leptos_router::components::A;
use leptos_router::hooks::use_location;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use crate::components::ascii::AsciiDivider;
use crate::lenis::use_lenis;
use crate::motion::{prefers_reduced_motion, use_scroll_y};

/// `leftItems` in the JSX.
const LEFT_ITEMS: [(&str, &str); 2] = [("/projects", "Projects"), ("/blog", "Blog")];

/// One half of the wordmark crossfade, and the mobile menu's fade.
const SWAP_S: f64 = 0.25;

/// `{ opacity: 0, y } -> { opacity: 1, y: 0 }` as an inline style, with Framer's default
/// tween easing (`easeOut`).
fn fade_up(visible: bool, y: f64, dur: f64, delay: f64) -> String {
    let t = format!(
        "transition: opacity {dur}s ease-out {delay}s, transform {dur}s ease-out {delay}s; will-change: opacity, transform;"
    );
    if visible {
        format!("opacity: 1; transform: translateY(0px); {t}")
    } else {
        format!("opacity: 0; transform: translateY({y}px); {t}")
    }
}

/// One half of the `AnimatePresence mode="wait"` wordmark crossfade.
///
/// Framer unmounts the outgoing span and mounts the incoming one, so each gets a fresh
/// `initial` (y: 10, below the baseline) even though it exits upward (y: -10). Both spans
/// are permanently mounted here, so the hidden state has to be re-armed by hand: the span
/// slides up and out over `SWAP_S`, and once that exit has finished it is silently parked
/// back below the baseline with no transition, ready to slide up into view again.
///
/// The incoming span waits `SWAP_S` before entering — that is what `mode="wait"` does.
fn wordmark_style(visible: Signal<bool>, reduced: bool) -> Signal<String> {
    let parked = RwSignal::new(!visible.get_untracked());

    if !reduced {
        Effect::new(move |prev: Option<bool>| {
            let now = visible.get();
            if prev == Some(now) {
                return now;
            }
            // Either way the span animates from where it currently sits, so drop the park.
            parked.set(false);
            if !now {
                // Exiting: slide up to -10, then re-arm below the baseline.
                set_timeout(
                    move || {
                        if !visible.get_untracked() {
                            parked.set(true);
                        }
                    },
                    Duration::from_millis((SWAP_S * 1000.0) as u64),
                );
            }
            now
        });
    }

    Signal::derive(move || {
        if reduced {
            return if visible.get() {
                "opacity: 1;".to_string()
            } else {
                "opacity: 0;".to_string()
            };
        }
        if visible.get() {
            format!(
                "opacity: 1; transform: translateY(0px); transition: opacity {SWAP_S}s ease-out {SWAP_S}s, transform {SWAP_S}s ease-out {SWAP_S}s;"
            )
        } else if parked.get() {
            "opacity: 0; transform: translateY(10px); transition: none;".to_string()
        } else {
            format!(
                "opacity: 0; transform: translateY(-10px); transition: opacity {SWAP_S}s ease-out, transform {SWAP_S}s ease-out;"
            )
        }
    })
}

/// lucide `menu`, at `size={17} strokeWidth={1.5}`.
#[component]
fn MenuIcon() -> impl IntoView {
    view! {
        <svg
            width="17"
            height="17"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="1.5"
            stroke-linecap="round"
            stroke-linejoin="round"
            aria-hidden="true"
        >
            <line x1="4" x2="20" y1="6" y2="6"></line>
            <line x1="4" x2="20" y1="12" y2="12"></line>
            <line x1="4" x2="20" y1="18" y2="18"></line>
        </svg>
    }
}

/// lucide `x`, at `size={17} strokeWidth={1.5}`.
#[component]
fn CloseIcon() -> impl IntoView {
    view! {
        <svg
            width="17"
            height="17"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="1.5"
            stroke-linecap="round"
            stroke-linejoin="round"
            aria-hidden="true"
        >
            <path d="M18 6 6 18"></path>
            <path d="m6 6 12 12"></path>
        </svg>
    }
}

/// lucide `github`, at `size={12} strokeWidth={1.5}`.
#[component]
fn GithubIcon() -> impl IntoView {
    view! {
        <svg
            width="12"
            height="12"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="1.5"
            stroke-linecap="round"
            stroke-linejoin="round"
            aria-hidden="true"
        >
            <path d="M15 22v-4a4.8 4.8 0 0 0-1-3.5c3 0 6-2 6-5.5.08-1.25-.27-2.48-1-3.5.28-1.15.28-2.35 0-3.5 0 0-1 0-3 1.5-2.64-.5-5.36-.5-8 0C6 2 5 2 5 2c-.3 1.15-.3 2.35 0 3.5A5.403 5.403 0 0 0 4 9c0 3.5 3 5.5 6 5.5-.39.49-.68 1.05-.85 1.65-.17.6-.22 1.23-.15 1.85v4"></path>
            <path d="M9 18c-4.51 2-5-2-7-2"></path>
        </svg>
    }
}

/// The arrow that slides in on hover in the mobile menu.
#[component]
fn ArrowIcon() -> impl IntoView {
    view! {
        <svg
            width="13"
            height="13"
            viewBox="0 0 12 12"
            fill="none"
            class="shrink-0 translate-y-px opacity-0 -translate-x-1 group-hover:opacity-100 group-hover:translate-x-0 transition-all duration-300"
        >
            <path
                d="M1 11L11 1M11 1H3M11 1V9"
                stroke="currentColor"
                stroke-width="1.2"
                stroke-linecap="round"
                stroke-linejoin="round"
            ></path>
        </svg>
    }
}

#[component]
pub fn Navbar() -> impl IntoView {
    let reduced = prefers_reduced_motion();
    let location = use_location();
    let lenis = use_lenis();

    let scroll_y = use_scroll_y();
    let scrolled = Signal::derive(move || scroll_y.get() > 60.0);

    let menu_open = RwSignal::new(false);
    // Leptos has no `AnimatePresence`, so the overlay is held in the DOM for `SWAP_S`
    // after closing to let it fade out, then unmounted by hand.
    let menu_mounted = RwSignal::new(false);
    let menu_shown = RwSignal::new(false);
    let menu_entered = RwSignal::new(false);

    // `initial={{ opacity: 0 }} animate={{ opacity: 1 }}` on the <nav>.
    let nav_mounted = RwSignal::new(reduced);
    if !reduced {
        request_animation_frame(move || nav_mounted.set(true));
    }

    // Route change closes the menu (the `currentPath !== prevPath` render-phase reset).
    Effect::new(move |prev: Option<String>| {
        let path = location.pathname.get();
        if let Some(prev) = prev {
            if prev != path {
                menu_open.set(false);
            }
        }
        path
    });

    // Freeze the page while the menu is open, and close it on Escape.
    let key_handler: Rc<RefCell<Option<Closure<dyn FnMut(web_sys::KeyboardEvent)>>>> =
        Rc::new(RefCell::new(None));
    Effect::new(move |_| {
        let open = menu_open.get();
        let body = document().body();

        if open {
            if let Some(l) = lenis {
                l.stop();
            }
            if let Some(b) = &body {
                let _ = b.style().set_property("overflow", "hidden");
            }
            let cb = Closure::<dyn FnMut(web_sys::KeyboardEvent)>::new(
                move |e: web_sys::KeyboardEvent| {
                    if e.key() == "Escape" {
                        menu_open.set(false);
                    }
                },
            );
            let _ =
                window().add_event_listener_with_callback("keydown", cb.as_ref().unchecked_ref());
            *key_handler.borrow_mut() = Some(cb);
        } else {
            if let Some(l) = lenis {
                l.start();
            }
            if let Some(b) = &body {
                let _ = b.style().remove_property("overflow");
            }
            if let Some(cb) = key_handler.borrow_mut().take() {
                let _ = window()
                    .remove_event_listener_with_callback("keydown", cb.as_ref().unchecked_ref());
            }
        }
    });

    // mount -> fade in -> (on close) fade out -> unmount.
    Effect::new(move |_| {
        if menu_open.get() {
            menu_entered.set(false);
            menu_shown.set(reduced);
            menu_mounted.set(true);
            if reduced {
                menu_entered.set(true);
            } else {
                request_animation_frame(move || {
                    if menu_open.get_untracked() {
                        menu_shown.set(true);
                        menu_entered.set(true);
                    }
                });
            }
        } else {
            menu_shown.set(false);
            if reduced {
                menu_mounted.set(false);
            } else {
                set_timeout(
                    move || {
                        if !menu_open.get_untracked() {
                            menu_mounted.set(false);
                        }
                    },
                    Duration::from_millis((SWAP_S * 1000.0) as u64),
                );
            }
        }
    });

    let full_style = wordmark_style(Signal::derive(move || !scrolled.get()), reduced);
    let initials_style = wordmark_style(Signal::derive(move || scrolled.get()), reduced);

    view! {
        <nav
            class=move || {
                if scrolled.get() || menu_open.get() {
                    "fixed top-0 left-0 right-0 z-50 transition-all duration-300 nav-solid"
                } else {
                    "fixed top-0 left-0 right-0 z-50 transition-all duration-300 bg-transparent"
                }
            }
            style=move || {
                if reduced {
                    String::new()
                } else if nav_mounted.get() {
                    "opacity: 1; transition: opacity 0.8s ease-out 0.2s;".to_string()
                } else {
                    "opacity: 0; transition: opacity 0.8s ease-out 0.2s;".to_string()
                }
            }
        >
            <div class="max-w-[1200px] mx-auto px-5 md:px-8">
                <div class=move || {
                    if scrolled.get() {
                        "grid grid-cols-[1fr_auto_1fr] items-center transition-all duration-300 h-14 md:h-16"
                    } else {
                        "grid grid-cols-[1fr_auto_1fr] items-center transition-all duration-300 h-16 md:h-20"
                    }
                }>
                    // Left
                    <div class="flex items-center gap-1 justify-self-start">
                        <button
                            type="button"
                            on:click=move |_| menu_open.update(|v| *v = !*v)
                            aria-label=move || {
                                if menu_open.get() { "Close menu" } else { "Open menu" }
                            }
                            aria-expanded=move || menu_open.get().to_string()
                            aria-controls="mobile-menu"
                            class="nav-chip sm:hidden -ml-2 flex items-center justify-center w-10 h-10 rounded-full text-muted hover:text-accent-light transition-colors"
                        >
                            {move || {
                                if menu_open.get() {
                                    view! { <CloseIcon /> }.into_any()
                                } else {
                                    view! { <MenuIcon /> }.into_any()
                                }
                            }}
                        </button>
                        {LEFT_ITEMS
                            .into_iter()
                            .map(|(to, label)| {
                                let active = Signal::derive(move || location.pathname.get() == to);
                                view! {
                                    <A
                                        href=to
                                        attr:class=move || {
                                            if active.get() {
                                                "nav-chip relative font-mono text-[10px] tracking-[0.18em] uppercase transition-colors hidden sm:block px-3 py-1.5 rounded-full text-cream"
                                            } else {
                                                "nav-chip relative font-mono text-[10px] tracking-[0.18em] uppercase transition-colors hidden sm:block px-3 py-1.5 rounded-full text-muted hover:text-accent-light"
                                            }
                                        }
                                    >
                                        <Show when=move || active.get()>
                                            <span class="nav-pill absolute inset-0 rounded-full -z-10"></span>
                                        </Show>
                                        {label}
                                    </A>
                                }
                            })
                            .collect_view()}
                    </div>

                    // Center — wordmark
                    <div class="flex flex-col items-center gap-1.5 justify-self-center">
                        <A
                            href="/"
                            attr:class="text-cream hover:text-accent-light transition-colors relative overflow-hidden grid justify-items-center"
                        >
                            <span
                                class="col-start-1 row-start-1 inline-block whitespace-nowrap font-mono text-[11px] sm:text-[12px] md:text-[13px] tracking-[0.16em] sm:tracking-[0.25em] uppercase font-medium"
                                style=full_style
                            >
                                "Teddy Tennant"
                            </span>
                            <span
                                class="col-start-1 row-start-1 inline-block whitespace-nowrap font-mono text-[11px] sm:text-[12px] md:text-[13px] tracking-[0.16em] sm:tracking-[0.25em] uppercase font-medium"
                                style=initials_style
                            >
                                "T.T."
                            </span>
                        </A>
                        <Show when=move || !scrolled.get()>
                            <div class="hidden md:flex items-center gap-2.5">
                                <a
                                    href="https://github.com/teddytennant"
                                    target="_blank"
                                    rel="noopener noreferrer"
                                    aria-label="GitHub"
                                    class="text-muted/50 hover:text-accent-light transition-colors"
                                >
                                    <GithubIcon />
                                </a>
                            </div>
                        </Show>
                    </div>

                    // Right
                    <div class="flex items-center gap-1 justify-self-end">
                        <a
                            href="https://github.com/teddytennant"
                            target="_blank"
                            rel="noopener noreferrer"
                            class="nav-chip font-mono text-[10px] tracking-[0.18em] uppercase text-muted hover:text-accent-light transition-colors hidden sm:block px-3 py-1.5 rounded-full"
                        >
                            "GitHub"
                        </a>
                        <a
                            href="mailto:teddy5tennant@gmail.com"
                            class="nav-chip font-mono text-[10px] tracking-[0.18em] uppercase text-muted hover:text-accent-light transition-colors px-3 py-2.5 sm:py-1.5 -mr-1.5 sm:mr-0 rounded-full"
                        >
                            "Contact"
                        </a>
                    </div>
                </div>
            </div>
        </nav>

        <Show when=move || menu_mounted.get()>
            <div
                id="mobile-menu"
                class="fixed inset-0 z-40 sm:hidden bg-ink/98 backdrop-blur-2xl flex flex-col pt-24 px-6 pb-8"
                style=move || {
                    if reduced {
                        String::new()
                    } else if menu_shown.get() {
                        format!("opacity: 1; transition: opacity {SWAP_S}s ease-out;")
                    } else {
                        format!("opacity: 0; transition: opacity {SWAP_S}s ease-out;")
                    }
                }
            >
                <nav aria-label="Mobile" class="flex-1 flex flex-col justify-center gap-0.5">
                    {LEFT_ITEMS
                        .into_iter()
                        .enumerate()
                        .map(|(i, (to, label))| {
                            let active = Signal::derive(move || location.pathname.get() == to);
                            let delay = 0.08 + i as f64 * 0.06;
                            view! {
                                <div style=move || {
                                    if reduced {
                                        String::new()
                                    } else {
                                        fade_up(menu_entered.get(), 16.0, 0.4, delay)
                                    }
                                }>
                                    <A
                                        href=to
                                        on:click=move |_| menu_open.set(false)
                                        attr:class=move || {
                                            if active.get() {
                                                "group flex items-center gap-4 py-4 border-b border-line text-cream"
                                            } else {
                                                "group flex items-center gap-4 py-4 border-b border-line text-cream/80"
                                            }
                                        }
                                    >
                                        <span class="font-mono text-[10px] text-muted/40 tabular-nums">
                                            {format!("{:02}", i + 1)}
                                        </span>
                                        <span class="font-serif font-black text-[clamp(2.2rem,12vw,3rem)] leading-none tracking-[-0.02em]">
                                            {label}
                                        </span>
                                        <ArrowIcon />
                                    </A>
                                </div>
                            }
                        })
                        .collect_view()}
                </nav>

                <AsciiDivider seed=2 dark=true class="-my-2" />

                <div
                    class="flex items-center justify-between"
                    style=move || {
                        if reduced {
                            String::new()
                        } else {
                            fade_up(menu_entered.get(), 12.0, 0.4, 0.2)
                        }
                    }
                >
                    <a
                        href="https://github.com/teddytennant"
                        target="_blank"
                        rel="noopener noreferrer"
                        on:click=move |_| menu_open.set(false)
                        class="font-mono text-[10px] tracking-[0.18em] uppercase text-muted hover:text-accent-light transition-colors"
                    >
                        "GitHub"
                    </a>
                    <a
                        href="mailto:teddy5tennant@gmail.com"
                        on:click=move |_| menu_open.set(false)
                        class="font-mono text-[10px] tracking-[0.18em] uppercase text-muted hover:text-accent-light transition-colors"
                    >
                        "Contact"
                    </a>
                </div>
            </div>
        </Show>
    }
}
