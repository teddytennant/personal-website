//! Port of src/pages/BlogPage.jsx

use leptos::prelude::*;
use leptos_router::hooks::use_navigate;

use crate::components::ascii::AsciiDivider;
use crate::format::roman_date;
use crate::motion::{prefers_reduced_motion, EASE_ENTER};

struct Post {
    title: &'static str,
    description: &'static str,
    date: &'static str,
    /// Relative path to a static HTML file under `public/` — not an SPA route.
    file: &'static str,
}

/// The `posts` array from BlogPage.jsx, verbatim.
const POSTS: &[Post] = &[
    Post {
        title: "The Harness Is the Frontier",
        description: "Frontier models are converging; the scaffolding around them isn't. Reflections on the Agentic Harness Engineering paper and applying its loop to wizard and FUSION.",
        date: "2026.07",
        file: "essays/harness-engineering.html",
    },
    Post {
        title: "Constitution for Truth-Seeking AI",
        description: "Six principles for models that report only what the evidence supports, written to be dropped in as a system prompt or used to steer synthetic data generation.",
        date: "2026.03",
        file: "essays/ai-governance-framework.html",
    },
    Post {
        title: "Full HOOTL Recursive Self-Improvement is Becoming Viable",
        description: "Recursive self-improvement with no human in the loop is starting to look practical. The constraint that remains is energy, not model capability.",
        date: "2026.02",
        file: "essays/hootl-rsi.html",
    },
    Post {
        title: "A Dimensional Classification of Intelligence",
        description: "A framework that classifies minds by their creator rather than their capability, from evolved brains to human-built models to AI-built successors.",
        date: "2025.11",
        file: "essays/dimensional-classification-essay.html",
    },
];

/// Framer's `initial={{ opacity: 0, y }} animate={{ opacity: 1, y: 0 }}` with a
/// per-item `delay`: render in the "before" state, then flip to rest once the
/// browser has painted, so the CSS transition (delay included) actually runs.
///
/// `motion::use_enter` hard-codes the home page's timing (y 18, 0.7s, 0.15 + 0.07i),
/// which is not this page's, so this is the local equivalent.
fn use_mount(y: f64, dur: f64, delay: f64) -> Signal<String> {
    let reduced = prefers_reduced_motion();
    let visible = RwSignal::new(reduced);

    if !reduced {
        request_animation_frame(move || {
            set_timeout(move || visible.set(true), std::time::Duration::from_millis(0));
        });
    }

    Signal::derive(move || {
        if reduced {
            return String::new();
        }
        let t = format!(
            "transition: opacity {dur}s {EASE_ENTER} {delay:.3}s, transform {dur}s {EASE_ENTER} {delay:.3}s; will-change: opacity, transform;"
        );
        if visible.get() {
            format!("opacity: 1; transform: translateY(0px); {t}")
        } else {
            format!("opacity: 0; transform: translateY({y}px); {t}")
        }
    })
}

#[component]
pub fn BlogPage() -> impl IntoView {
    let navigate = use_navigate();
    let header_style = use_mount(16.0, 0.7, 0.0);

    view! {
        <div class="pt-28 md:pt-36 pb-24 px-6 md:px-10">
            <div class="max-w-[860px] mx-auto">
                // Header
                <div style=header_style class="mb-16 md:mb-20">
                    <button
                        on:click=move |_| navigate("/", Default::default())
                        class="font-mono text-[11px] text-muted hover:text-accent-light transition-colors tracking-[0.05em] mb-8 inline-block"
                    >
                        "← Home"
                    </button>
                    <p class="font-mono text-[10px] text-muted/70 tracking-[0.25em] uppercase mb-4 flex items-center gap-1.5">
                        <span class="w-1 h-1 bg-accent/80 inline-block shrink-0"></span>
                        "Writing"
                    </p>
                    <h1 class="font-serif font-black text-[clamp(2.6rem,5vw,4rem)] leading-[1.05] tracking-[-0.02em] text-cream mb-3">
                        "Blog"
                    </h1>
                </div>

                <AsciiDivider seed=3 dark=true class="-mb-2" />

                // List
                <div>
                    {POSTS
                        .iter()
                        .enumerate()
                        .map(|(i, post)| {
                            let row_style = use_mount(12.0, 0.6, i as f64 * 0.06);
                            view! {
                                // The essays are static HTML files under public/, not SPA routes,
                                // so this has to be a real browser navigation. React Router leaves
                                // plain anchors alone; leptos_router intercepts same-origin clicks
                                // unless `rel="external"`, hence the extra rel.
                                <a
                                    href=post.file
                                    rel="external"
                                    style=row_style
                                    class="group row-glass border-b border-line py-7 md:py-8 px-4 md:px-6 -mx-4 md:-mx-6 rounded-md flex flex-col md:flex-row md:items-baseline md:justify-between gap-2 md:gap-8"
                                >
                                    <div class="min-w-0">
                                        <p class="font-mono text-[10px] text-muted tracking-[0.15em] uppercase mb-2">
                                            {roman_date(post.date)}
                                        </p>
                                        <h2 class="title-hover text-[18px] md:text-[19px] text-cream font-medium tracking-tight">
                                            {post.title}
                                        </h2>
                                        <p class="text-[14px] text-muted leading-[1.65] font-light mt-2 max-w-xl">
                                            {post.description}
                                        </p>
                                    </div>
                                    <span class="font-mono text-[10px] text-muted group-hover:text-accent-light transition-colors shrink-0 tracking-[0.08em]">
                                        "Read →"
                                    </span>
                                </a>
                            }
                        })
                        .collect_view()}
                </div>
            </div>
        </div>
    }
}
