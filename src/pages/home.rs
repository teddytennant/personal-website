//! Port of src/pages/home/HomePage.jsx.

use leptos::html::{Div, A as AElem};
use leptos::prelude::*;
use leptos_router::components::A;

use crate::components::ascii::AsciiPortrait;
use crate::content::{CONTACTS, ESSAYS, FEATURED, PROJECTS};
use crate::format::roman_date;
use crate::media::use_media_query;
use crate::motion::{map_range, prefers_reduced_motion, use_enter, use_reveal, use_scroll_y};

#[component]
fn ArrowIcon() -> impl IntoView {
    view! {
        <svg
            width="11"
            height="11"
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
            />
        </svg>
    }
}

#[component]
fn CellLabel(children: Children) -> impl IntoView {
    view! {
        <p class="font-mono text-[9px] text-muted/70 tracking-[0.25em] uppercase flex items-center gap-1.5">
            <span class="w-1 h-1 bg-accent/80 inline-block shrink-0" />
            {children()}
        </p>
    }
}

#[component]
fn TerminalBox() -> impl IntoView {
    view! {
        <div class="panel mt-7 max-w-md font-mono text-[11px] md:text-[12px] leading-relaxed">
            <div class="flex items-center gap-1.5 px-4 md:px-5 py-2.5 border-b border-accent/40">
                <span class="w-2 h-2 rounded-full border border-cream/60" />
                <span class="w-2 h-2 rounded-full border border-cream/60" />
                <span class="w-2 h-2 rounded-full border border-cream/60" />
            </div>
            <div class="p-4 md:p-5">
                <p class="text-muted">
                    <span class="text-accent-light">"$"</span>
                    " whoami"
                </p>
                <p class="text-cream/90 mb-3">"teddy tennant · ncssm ‘28 · engineer"</p>
                <p class="text-muted">
                    <span class="text-accent-light">"$"</span>
                    " status --current"
                </p>
                <p class="text-cream/90">
                    "reasoning in latent space"
                    <span class="cursor-blink" />
                </p>
            </div>
        </div>
    }
}

/// `useScroll` + `useTransform(scrollY, [0, 120], [1, 0])`.
#[component]
fn ScrollCue() -> impl IntoView {
    let scroll_y = use_scroll_y();
    let opacity = move || {
        format!(
            "opacity: {};",
            map_range(scroll_y.get(), (0.0, 120.0), (1.0, 0.0))
        )
    };

    view! {
        <div
            style=opacity
            class="absolute bottom-6 left-1/2 -translate-x-1/2 flex flex-col items-center gap-2.5 pointer-events-none"
        >
            <span class="font-mono text-[9px] text-muted/60 tracking-[0.3em] uppercase">
                "Scroll"
            </span>
            <svg
                width="10"
                height="24"
                viewBox="0 0 10 24"
                fill="none"
                class="scroll-cue-arrow text-muted/70"
            >
                <path
                    d="M5 1V22M5 22L1 17.5M5 22L9 17.5"
                    stroke="currentColor"
                    stroke-width="1"
                    stroke-linecap="round"
                    stroke-linejoin="round"
                />
            </svg>
        </div>
    }
}

#[component]
pub fn HomePage() -> impl IntoView {
    let is_desktop = use_media_query("(min-width: 768px)");

    // Hero: `container` orchestrates two `cell` children (delayChildren 0.15,
    // staggerChildren 0.07).
    let identity_style = use_enter(0);
    let portrait_style = use_enter(1);

    // `container` also fades *itself* in (hidden: { opacity: 0 } -> visible: { opacity: 1 }).
    // The ScrollCue isn't a variant child, so this container fade is the only thing that
    // brings it in — without it the cue pops in at full opacity on first paint.
    let reduced = prefers_reduced_motion();
    let hero_shown = RwSignal::new(reduced);
    if !reduced {
        request_animation_frame(move || hero_shown.set(true));
    }
    let hero_style = move || {
        if reduced {
            String::new()
        } else if hero_shown.get() {
            "opacity: 1; transition: opacity 0.3s ease;".to_string()
        } else {
            "opacity: 0; transition: opacity 0.3s ease;".to_string()
        }
    };

    // `heroRef` in the JSX — the 440px portrait cell that AsciiPortrait's
    // `useScroll({ target, offset: ['start start', 'end start'] })` tracks.
    let hero_ref = NodeRef::<Div>::new();

    // Bento cells each reveal on their own (the grid parent is a plain div, so
    // there is no stagger between them).
    let featured_ref = NodeRef::<AElem>::new();
    let featured_style = use_reveal(featured_ref);
    let work_ref = NodeRef::<Div>::new();
    let work_style = use_reveal(work_ref);
    let about_ref = NodeRef::<Div>::new();
    let about_style = use_reveal(about_ref);
    let blog_ref = NodeRef::<Div>::new();
    let blog_style = use_reveal(blog_ref);
    let contact_ref = NodeRef::<Div>::new();
    let contact_style = use_reveal(contact_ref);

    view! {
        <div class="relative px-5 md:px-8 pb-8 overflow-hidden">
            <div class="relative z-10 max-w-[1200px] mx-auto w-full">
                // ── Hero split — fills the viewport so the grid starts below the fold ──
                <div
                    style=hero_style
                    class="relative min-h-[100dvh] grid md:grid-cols-[1.15fr_1fr] gap-10 md:gap-16 items-center content-center pt-24 pb-10"
                >
                    // Left — identity
                    <div style=identity_style>
                        <h1 class="font-serif font-black text-[clamp(2.8rem,7vw,5.5rem)] leading-[0.92] tracking-[-0.03em] text-cream mb-5">
                            "Teddy Tennant"
                        </h1>
                        <p class="text-[19px] md:text-[22px] text-cream font-medium leading-snug mb-4 max-w-md">
                            "AI engineer."
                        </p>
                        <p class="text-[14px] text-muted leading-relaxed max-w-md font-light">
                            "16 year old at NCSSM. Working on RL environments, synthetic data, and "
                            <span class="text-accent-light font-normal">
                                "recursive self-improvement"
                            </span>
                            "."
                        </p>
                        <TerminalBox />
                    </div>

                    // Right — ASCII graphic, desktop only
                    {move || {
                        is_desktop
                            .get()
                            .then(|| {
                                view! {
                                    <div
                                        node_ref=hero_ref
                                        style=portrait_style
                                        class="relative h-[440px] overflow-hidden"
                                    >
                                        <div class="absolute inset-0">
                                            <AsciiPortrait scroll_ref=hero_ref src="/pfp-ascii.png" />
                                        </div>
                                    </div>
                                }
                            })
                    }}

                    <ScrollCue />
                </div>

                // ── Bento grid ───────────────────────────────
                <div class="grid grid-cols-1 md:grid-cols-12 gap-3">

                    // Featured: Wizard
                    <a
                        node_ref=featured_ref
                        style=featured_style
                        href=FEATURED.href
                        target="_blank"
                        rel="noopener noreferrer"
                        class="group panel panel-hover md:col-span-7 md:row-span-2 p-6 md:p-8 flex flex-col justify-between relative overflow-hidden"
                    >
                        <div class="flex items-center justify-between mb-6 relative">
                            <CellLabel>"Featured Project"</CellLabel>
                            <span class="font-mono text-[9px] text-muted/50 tracking-[0.15em] uppercase hidden sm:block">
                                {FEATURED.tags.join(" / ")}
                            </span>
                        </div>
                        <div class="relative">
                            <h2 class="font-serif font-black text-[clamp(2.4rem,5vw,4.4rem)] leading-[0.95] tracking-[-0.03em] text-cream mb-4">
                                {FEATURED.title}
                            </h2>
                            <p class="text-[14px] md:text-[15px] text-muted leading-[1.75] font-light max-w-xl mb-6">
                                {FEATURED.description}
                            </p>
                            <span class="font-mono text-[10px] text-accent-light group-hover:text-cream transition-colors tracking-[0.08em] inline-flex items-center gap-2">
                                {FEATURED.link_label}
                                <ArrowIcon />
                            </span>
                        </div>
                    </a>

                    // Project list
                    <div
                        node_ref=work_ref
                        style=work_style
                        class="panel panel-hover md:col-span-5 md:row-span-2 p-6 flex flex-col"
                    >
                        <div class="flex items-baseline justify-between mb-4">
                            <CellLabel>"Selected Work"</CellLabel>
                            <A
                                href="/projects"
                                attr:class="font-mono text-[10px] text-muted/60 hover:text-accent-light transition-colors tracking-[0.05em]"
                            >
                                "All →"
                            </A>
                        </div>
                        <div class="divide-y divide-cream/25 flex-1">
                            {PROJECTS
                                .iter()
                                .map(|project| match project.href {
                                    Some(href) => {
                                        view! {
                                            <a
                                                href=href
                                                target="_blank"
                                                rel="noopener noreferrer"
                                                class="group flex items-baseline justify-between gap-4 py-3 first:pt-0 last:pb-0"
                                            >
                                                <div class="flex items-baseline gap-3 min-w-0">
                                                    <h3 class="title-hover text-[15px] text-cream font-sans font-medium tracking-tight shrink-0">
                                                        {project.title}
                                                    </h3>
                                                    <span class="text-[12px] text-muted/80 font-light truncate hidden sm:block">
                                                        {project.description}
                                                    </span>
                                                </div>
                                                <span class="text-muted/60 group-hover:text-accent-light transition-colors">
                                                    <ArrowIcon />
                                                </span>
                                            </a>
                                        }
                                            .into_any()
                                    }
                                    None => {
                                        view! {
                                            <div class="group flex items-baseline justify-between gap-4 py-3 first:pt-0 last:pb-0">
                                                <div class="flex items-baseline gap-3 min-w-0">
                                                    <h3 class="title-hover text-[15px] text-cream font-sans font-medium tracking-tight shrink-0">
                                                        {project.title}
                                                    </h3>
                                                    <span class="text-[12px] text-muted/80 font-light truncate hidden sm:block">
                                                        {project.description}
                                                    </span>
                                                </div>
                                                <span class="font-mono text-[8px] text-muted/40 tracking-[0.15em] uppercase shrink-0">
                                                    "Private"
                                                </span>
                                            </div>
                                        }
                                            .into_any()
                                    }
                                })
                                .collect_view()}
                        </div>
                    </div>

                    // About
                    <div
                        node_ref=about_ref
                        style=about_style
                        class="panel panel-hover md:col-span-5 p-6"
                    >
                        <CellLabel>"About"</CellLabel>
                        <p class="font-serif italic text-[18px] md:text-[20px] leading-snug text-cream mt-3 mb-3">
                            "I make software that figures things out on its own."
                        </p>
                        <p class="text-[13px] text-muted leading-[1.75] font-light">
                            "Agent harness engineering, honesty & calibration evals, and multi-agent debate frameworks, mostly in Rust and Python. Ship working systems first, optimize later; the best tools are often the ones you build yourself."
                        </p>
                    </div>

                    // Blog
                    <div
                        node_ref=blog_ref
                        style=blog_style
                        class="panel panel-hover md:col-span-4 p-6 flex flex-col"
                    >
                        <div class="flex items-baseline justify-between mb-4">
                            <CellLabel>"Blog"</CellLabel>
                            <A
                                href="/blog"
                                attr:class="font-mono text-[10px] text-muted/60 hover:text-accent-light transition-colors tracking-[0.05em]"
                            >
                                "All →"
                            </A>
                        </div>
                        <div class="divide-y divide-cream/25 flex-1">
                            {ESSAYS
                                .iter()
                                .map(|essay| {
                                    view! {
                                        <A
                                            href="/blog"
                                            attr:class="group flex items-baseline justify-between gap-3 py-2.5 first:pt-0 last:pb-0"
                                        >
                                            <span class="title-hover text-[13px] text-cream/90 font-light leading-snug">
                                                {essay.title}
                                            </span>
                                            <span class="font-mono text-[9px] text-muted/50 tracking-[0.1em] shrink-0">
                                                {roman_date(essay.date)}
                                            </span>
                                        </A>
                                    }
                                })
                                .collect_view()}
                        </div>
                    </div>

                    // Contact
                    <div
                        node_ref=contact_ref
                        style=contact_style
                        class="panel panel-hover md:col-span-3 p-6 flex flex-col"
                    >
                        <CellLabel>"Contact"</CellLabel>
                        <div class="mt-4 space-y-2.5">
                            {CONTACTS
                                .iter()
                                .map(|link| {
                                    let target = (!link.href.starts_with("mailto"))
                                        .then_some("_blank");
                                    view! {
                                        <a
                                            href=link.href
                                            target=target
                                            rel="noopener noreferrer"
                                            class="group flex items-baseline justify-between gap-3"
                                        >
                                            <span class="font-mono text-[9px] text-muted/50 tracking-[0.15em] uppercase">
                                                {link.label}
                                            </span>
                                            <span class="font-mono text-[11px] text-cream/80 group-hover:text-accent-light transition-colors truncate">
                                                {link.value}
                                            </span>
                                        </a>
                                    }
                                })
                                .collect_view()}
                        </div>
                        <p class="mt-auto pt-4 font-mono text-[9px] text-muted/40 tracking-[0.15em] uppercase flex items-center gap-1.5">
                            <span class="w-1 h-1 rounded-full bg-accent glow-dot" />
                            "Available for hire"
                        </p>
                    </div>
                </div>

                // overengineeRING webring embed
                <div class="mt-3 panel p-0 overflow-hidden">
                    <iframe
                        src="https://overengineering.kognise.dev/embed/teddy"
                        title="overengineeRING embed"
                        width="100%"
                        height="100"
                        style="user-select: none; border: 0; display: block; background: transparent;"
                        frameborder="0"
                    />
                </div>
            </div>
        </div>
    }
}
