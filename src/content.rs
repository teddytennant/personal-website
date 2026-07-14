//! Port of src/content/home.js — the site's copy, verbatim.

pub struct Featured {
    pub title: &'static str,
    pub description: &'static str,
    pub tags: &'static [&'static str],
    pub href: &'static str,
    pub link_label: &'static str,
}

pub struct Project {
    pub title: &'static str,
    pub description: &'static str,
    /// `None` renders the "Private" tag instead of a link.
    pub href: Option<&'static str>,
}

pub struct Essay {
    pub title: &'static str,
    pub date: &'static str,
}

pub struct Contact {
    pub label: &'static str,
    pub value: &'static str,
    pub href: &'static str,
}

pub const FEATURED: Featured = Featured {
    title: "Wizard",
    description: "A self-extending autonomous coding agent in one Rust binary. Any provider (OpenAI-compatible, Anthropic, xAI) or fully local via llama.cpp. Live /evolve self-modification, MCP, a messaging gateway, and a built-in bench.",
    tags: &["Rust", "Agent Harness", "Self-Modifying"],
    href: "https://wizard.teddytennant.com",
    link_label: "wizard.teddytennant.com",
};

pub const PROJECTS: &[Project] = &[
    Project {
        title: "candor-bench",
        description: "Honesty, sycophancy, calibration, and factuality evals for LLMs.",
        href: Some("https://github.com/teddytennant/candor-bench"),
    },
    Project {
        title: "agentic-harness-engineering",
        description: "NexAU-AHE: 84.7% pass@1 on Terminal-Bench 2, beats Codex/ACE.",
        href: Some("https://github.com/teddytennant/agentic-harness-engineering"),
    },
    Project {
        title: "reverie",
        description: "Curriculum-free latent-space reasoning, a Coconut successor in JAX.",
        href: Some("https://github.com/teddytennant/reverie"),
    },
    Project {
        title: "spore",
        description: "Minimal self-rewriting coding agent, one bash tool, ~270 lines of Rust.",
        href: Some("https://github.com/teddytennant/spore"),
    },
];

pub const ESSAYS: &[Essay] = &[
    Essay {
        title: "Constitution for Truth-Seeking AI",
        date: "2026.03",
    },
    Essay {
        title: "Full HOOTL Recursive Self-Improvement",
        date: "2026.02",
    },
    Essay {
        title: "A Dimensional Classification of Intelligence",
        date: "2025.11",
    },
];

pub const CONTACTS: &[Contact] = &[
    Contact {
        label: "Email",
        value: "teddy5tennant@gmail.com",
        href: "mailto:teddy5tennant@gmail.com",
    },
    Contact {
        label: "GitHub",
        value: "@teddytennant",
        href: "https://github.com/teddytennant",
    },
    Contact {
        label: "HuggingFace",
        value: "ttennant",
        href: "https://huggingface.co/ttennant",
    },
];
