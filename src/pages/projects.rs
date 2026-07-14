//! Port of src/pages/ProjectsPage.jsx

use leptos::prelude::*;
use leptos_router::hooks::use_navigate;

use crate::components::ascii::AsciiDivider;
use crate::motion::{prefers_reduced_motion, EASE_ENTER};

struct Project {
    title: &'static str,
    description: &'static str,
    tags: &'static [&'static str],
    link: Option<&'static str>,
    link_label: Option<&'static str>,
}

/// The `projects` array from ProjectsPage.jsx, verbatim.
const PROJECTS: &[Project] = &[
    Project {
        title: "Wizard",
        description: "A self-extending autonomous coding agent in one Rust binary. Any provider — OpenAI-compatible, Anthropic, xAI — or fully local via llama.cpp, with live /evolve self-modification, MCP support, a messaging gateway, and a built-in bench. Live at wizard.teddytennant.com.",
        tags: &["Rust", "Agent Harness", "Self-Modifying"],
        link: Some("https://wizard.teddytennant.com"),
        link_label: Some("Site"),
    },
    Project {
        title: "spore",
        description: "The most minimal functional coding agent: one tool, bash, and no approval gate. It edits code, runs builds and tests, spawns subagents, and can rewrite and recompile its own source. Around 270 lines of Rust with two dependencies.",
        tags: &["Rust", "Coding Agent", "Self-Modifying"],
        link: Some("https://github.com/teddytennant/spore"),
        link_label: None,
    },
    Project {
        title: "reverie",
        description: "Adaptive, curriculum-free reasoning in a continuous latent space, a Coconut successor in JAX. Every continuous thought is supervised to decode to its gold reasoning step, and a depth-supervised differentiable halt calibrates latent compute to each problem's difficulty (rho ~ 1.0), no curriculum and no RL.",
        tags: &["JAX", "Latent Reasoning", "ML Research"],
        link: Some("https://github.com/teddytennant/reverie"),
        link_label: None,
    },
    Project {
        title: "audition",
        description: "Verifiable RL environments for agentic LLMs. A harness plus a pack of terminal and coding tasks where reward is computed from the final state of the world — files, exit codes, git history — never from a judge model. Sandboxed episodes, declarative verifiers, per-check audit trails.",
        tags: &["Python", "RL Environments", "Evals"],
        link: None,
        link_label: None,
    },
    Project {
        title: "candor-bench",
        description: "Eval suite for honesty-relevant failure modes in instruction-tuned LLMs: honesty, sycophancy, calibration, and factuality. Four deliberately disjoint axes across 143 items, never averaged — each family catches a failure the others can't.",
        tags: &["Python", "Evals", "Alignment"],
        link: Some("https://github.com/teddytennant/candor-bench"),
        link_label: None,
    },
    Project {
        title: "agentic-harness-engineering",
        description: "Observability-driven automatic evolution of coding-agent harnesses: run the agent over a Docker task set, analyze transcripts, rewrite the harness bundle — system prompt, tools, skills — and re-measure with automatic rollback. NexAU-AHE reaches 84.7% pass@1 on Terminal-Bench 2.",
        tags: &["Python", "Agent Harness", "Benchmarks"],
        link: Some("https://github.com/teddytennant/agentic-harness-engineering"),
        link_label: None,
    },
    Project {
        title: "lrd-reason",
        description: "Latent Recurrent Diffusion Reasoning: a persistent recurrent state and a latent rectified-flow refinement module bolted onto a frozen instruction-tuned LLM. Reasoning happens in a 256-dim latent space before generation; only a small LoRA adapter is trained.",
        tags: &["Python", "PyTorch", "ML Research"],
        link: Some("https://github.com/teddytennant/lrd-reason"),
        link_label: None,
    },
    Project {
        title: "sparse-attn",
        description: "Composable sparse-attention building blocks for long-context inference: chunk pooling, top-k routing, KV eviction, document-wise RoPE, and tiered memory as independent Rust modules you wire together — primitives, not a monolithic engine.",
        tags: &["Rust", "Attention", "Long Context"],
        link: Some("https://github.com/teddytennant/sparse-attn"),
        link_label: None,
    },
    Project {
        title: "moe-distill",
        description: "Mixture-of-Experts analysis and distillation toolkit: profile which experts fire on domain-specific prompts, find dead and domain-specialized experts, and distill the model down to the subset a target domain actually uses.",
        tags: &["Rust", "MoE", "ML Research"],
        link: Some("https://github.com/teddytennant/moe-distill"),
        link_label: None,
    },
    Project {
        title: "recursive-self-improvement",
        description: "Research demonstration of recursive LLM self-improvement through iterative fine-tuning. Includes Claude-as-Teacher and STaR algorithm modes, anti-forgetting mechanisms, and 322+ tests.",
        tags: &["Python", "PyTorch", "ML Research"],
        link: None,
        link_label: None,
    },
    Project {
        title: "rusttorch",
        description: "High-performance PyTorch extension in Rust with 55+ accelerated ML operations, SIMD optimization, rayon parallelism, and NumPy-compatible broadcasting. Targeting 1.2–2x CPU speedup over PyTorch C++.",
        tags: &["Rust", "PyTorch", "Performance"],
        link: Some("https://github.com/teddytennant/rusttorch"),
        link_label: None,
    },
    Project {
        title: "ferret",
        description: "Attack-search engine for the Kaggle AI Agent Security competition (OpenAI / Google / IEEE). Algorithmic red-teaming: a coverage-guided fuzzer over the competition's cell-signature space that discovers replayable multi-step tool attacks against a sandboxed agent.",
        tags: &["Python", "AI Security", "Red-Teaming"],
        link: None,
        link_label: None,
    },
    Project {
        title: "tempo",
        description: "Agent for the Pokémon TCG AI Battle Challenge (Kaggle × The Pokémon Company). A small ONNX policy ranks the engine's legal options, CPU-only inside a 10-minute game clock, with MCTS over a Rust forward model layered on top of the behavior-cloning baseline.",
        tags: &["Python", "Rust", "Game AI"],
        link: None,
        link_label: None,
    },
    Project {
        title: "axon",
        description: "Peer-to-peer runtime for AI agents to discover, communicate, and collaborate without central infrastructure. Built in Rust with QUIC transport, CRDTs for shared state, capability-based routing, and a TUI dashboard.",
        tags: &["Rust", "Distributed Systems", "P2P"],
        link: None,
        link_label: None,
    },
    Project {
        title: "fugue",
        description: "Security-first AI agent gateway in Rust connecting LLM providers (Ollama, Anthropic, OpenAI) to messaging channels (Telegram, Discord, IRC, Matrix) with WASM-sandboxed plugins and encrypted credential storage.",
        tags: &["Rust", "Security", "WASM"],
        link: None,
        link_label: None,
    },
    Project {
        title: "nous",
        description: "Decentralized everything-app in Rust: self-sovereign identity (DID:key), end-to-end encrypted messaging (Double Ratchet), quadratic on-chain governance, P2P payments, and a Nostr relay in one local-first protocol. Ships as CLI, API server, TUI, web, and desktop.",
        tags: &["Rust", "P2P", "Cryptography"],
        link: Some("https://github.com/teddytennant/nous"),
        link_label: None,
    },
    Project {
        title: "spectral",
        description: "Peer-to-peer, end-to-end encrypted file sync protocol in Rust. Every block is XChaCha20-Poly1305-encrypted before it leaves the device, and transport is a trait — LAN, relay, or first-class sneakernet via tamper-evident encrypted bundles.",
        tags: &["Rust", "Cryptography", "Sync"],
        link: None,
        link_label: None,
    },
    Project {
        title: "kryptos",
        description: "A vim-first Signal desktop client for Linux. Native GTK4 + libadwaita — no Electron, no webview — with a modal keybinding engine over signal-cli. Built for NixOS and Hyprland.",
        tags: &["Rust", "GTK4", "Linux"],
        link: Some("https://github.com/teddytennant/kryptos"),
        link_label: None,
    },
    Project {
        title: "PredLab",
        description: "Paper-trading platform for a school prediction-markets club: Polymarket-style yes/no markets, $25,000 of fake money, and a live auto-updating leaderboard. Rust backend with API-key clients. Live at predlab.teddytennant.com.",
        tags: &["Rust", "Prediction Markets", "Education"],
        link: Some("https://predlab.teddytennant.com"),
        link_label: Some("Site"),
    },
    Project {
        title: "Eternal",
        description: "Local-first RAG engine for Markdown knowledge bases, written in Zig. Compiles to a single zero-dependency binary, uses sparse TF-IDF embeddings, and runs in minimal RAM.",
        tags: &["Zig", "RAG", "Systems"],
        link: Some("https://github.com/teddytennant/Eternal"),
        link_label: None,
    },
    Project {
        title: "Fugo",
        description: "AI-powered radiology platform for autonomous diagnostic imaging in resource-limited healthcare settings. Full-stack with a React 19 frontend, Rust/Axum backend, DICOM support, HIPAA compliance, and ONNX/PyTorch inference.",
        tags: &["Rust", "React", "Medical AI"],
        link: None,
        link_label: None,
    },
    Project {
        title: "Principia AI Homeschool",
        description: "AI-powered homeschool learning platform with mastery-based progression, parent and teacher dashboards, Stripe payments, and native iOS/Android apps. Built with Next.js 15, Supabase, Swift, and Kotlin.",
        tags: &["Next.js", "Swift", "Kotlin", "Education"],
        link: None,
        link_label: None,
    },
    Project {
        title: "Aether",
        description: "Open-world sci-fi fantasy sandbox in Unreal Engine 5. C++ with ModularGameplay, MassEntity/MassAI for large-scale simulation, and the Gameplay Ability System. In active development.",
        tags: &["Unreal Engine 5", "C++", "Gamedev"],
        link: Some("https://github.com/teddytennant/aether"),
        link_label: None,
    },
    Project {
        title: "GhostDroid",
        description: "The first polished open-source Android SSH terminal powered by a self-contained C++20 VT100/VT220 state machine. Jetpack Compose UI with full hardware keyboard support and SSH key auth.",
        tags: &["Kotlin", "C++", "Android"],
        link: None,
        link_label: None,
    },
    Project {
        title: "CarpoolBee",
        description: "iOS carpool scheduling app with an iMessage extension for coordinating rides through group chats. Features recurring schedules, MapKit location search, CloudKit sync, and calendar integration.",
        tags: &["Swift", "SwiftUI", "iOS"],
        link: None,
        link_label: None,
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
pub fn ProjectsPage() -> impl IntoView {
    let navigate = use_navigate();
    let header_style = use_mount(16.0, 0.7, 0.0);

    view! {
        <div class="pt-24 md:pt-32 pb-20 px-6 md:px-10">
            <div class="max-w-[1400px] mx-auto">
                // Header
                <div
                    style=header_style
                    class="grid md:grid-cols-[280px_1fr] lg:grid-cols-[320px_1fr] gap-8 md:gap-20 mb-20"
                >
                    <div>
                        <p class="font-mono text-[10px] text-muted/70 tracking-[0.25em] uppercase mb-4 flex items-center gap-1.5">
                            <span class="w-1 h-1 bg-accent/80 inline-block shrink-0"></span>
                            "Selected Work"
                        </p>
                        <button
                            on:click=move |_| navigate("/", Default::default())
                            class="font-mono text-[11px] text-muted/60 hover:text-accent-light transition-colors tracking-[0.05em]"
                        >
                            "← Home"
                        </button>
                    </div>
                    <div>
                        <h1 class="font-serif font-black text-[clamp(2.5rem,5vw,4.5rem)] leading-[1.05] tracking-[-0.02em] text-cream mb-4">
                            "Projects"
                        </h1>
                        <p class="text-[15px] text-muted leading-relaxed max-w-lg font-light">
                            "A collection of work in AI, systems programming, and research."
                        </p>
                    </div>
                </div>

                <AsciiDivider seed=1 dark=true class="-mb-2" />

                // List
                <div class="divide-y divide-line">
                    {PROJECTS
                        .iter()
                        .enumerate()
                        .map(|(i, project)| {
                            let row_style = use_mount(12.0, 0.6, i as f64 * 0.06);

                            let inner = view! {
                                <div class="grid md:grid-cols-[280px_1fr] lg:grid-cols-[320px_1fr] gap-4 md:gap-20">
                                    <div class="flex items-baseline gap-4">
                                        <span class="font-mono text-[10px] text-muted/30 group-hover:text-accent-light transition-colors tabular-nums">
                                            {format!("{:02}", i + 1)}
                                        </span>
                                        <h2 class="title-hover text-[20px] md:text-[22px] text-cream font-medium tracking-tight">
                                            {project.title}
                                        </h2>
                                    </div>
                                    <div>
                                        <p class="text-[15px] text-muted leading-[1.7] mb-4 font-light">
                                            {project.description}
                                        </p>
                                        <div class="flex flex-wrap items-center gap-2">
                                            <span class="font-mono text-[9px] text-muted/50 tracking-[0.15em] uppercase">
                                                {project.tags.join(" / ")}
                                            </span>
                                            {project
                                                .link
                                                .map(|_| {
                                                    view! {
                                                        <span class="w-px h-3 bg-line mx-1"></span>
                                                        <span class="font-mono text-[10px] text-muted/50 group-hover:text-accent-light transition-colors flex items-center gap-1.5">
                                                            {project.link_label.unwrap_or("GitHub")}
                                                            <svg
                                                                width="12"
                                                                height="12"
                                                                viewBox="0 0 12 12"
                                                                fill="none"
                                                                class="translate-y-px opacity-0 -translate-x-1 group-hover:opacity-100 group-hover:translate-x-0 transition-all duration-300"
                                                            >
                                                                <path
                                                                    d="M1 11L11 1M11 1H3M11 1V9"
                                                                    stroke="currentColor"
                                                                    stroke-width="1.2"
                                                                    stroke-linecap="round"
                                                                    stroke-linejoin="round"
                                                                ></path>
                                                            </svg>
                                                        </span>
                                                    }
                                                })}
                                        </div>
                                    </div>
                                </div>
                            };

                            // `const Wrapper = project.link ? 'a' : 'div'`
                            let wrapper = match project.link {
                                Some(href) => {
                                    view! {
                                        <a
                                            href=href
                                            target="_blank"
                                            rel="noopener noreferrer"
                                            class="group row-glass block py-8 md:py-10 -mx-4 px-4 md:-mx-6 md:px-6 rounded-md"
                                        >
                                            {inner}
                                        </a>
                                    }
                                        .into_any()
                                }
                                None => {
                                    view! {
                                        <div class="group row-glass block py-8 md:py-10 -mx-4 px-4 md:-mx-6 md:px-6 rounded-md">
                                            {inner}
                                        </div>
                                    }
                                        .into_any()
                                }
                            };

                            view! { <div style=row_style>{wrapper}</div> }
                        })
                        .collect_view()}
                </div>
            </div>
        </div>
    }
}
