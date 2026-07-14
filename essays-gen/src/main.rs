//! Renders the standalone essay pages to static HTML.
//!
//! The essays are their own visual world: their own fonts and stylesheet, no
//! Tailwind, and none of the main site's chrome. So they are authored here in
//! Rust (via maud) and emitted as plain static pages, rather than folded into
//! the WASM app where the global styles would change how they render.
//!
//! Run by Trunk's pre_build hook:  cargo run -p essays-gen -- <out-dir>
//! (default out-dir: public/essays)

mod essays;

use std::fs;
use std::path::Path;

use maud::{Markup, PreEscaped, DOCTYPE};

/// Sets the theme class before first paint, so a light-mode reader gets no dark
/// flash. Identical to the inline script in the original essay pages.
const THEME_PREPAINT: &str = "try { var t = localStorage.getItem('theme'); if (t === 'light' || (!t && matchMedia('(prefers-color-scheme: light)').matches)) document.documentElement.classList.add('light') } catch (e) {}";

const FONTS_HREF: &str = "https://fonts.googleapis.com/css2?family=Instrument+Serif:ital@0;1&family=Inter:opsz,wght@14..32,300;14..32,400;14..32,500&display=swap";

/// The shared page shell. `title` is the `<title>` text (before the name suffix);
/// `body` is the content of `<article class="essay">`.
fn page(title: &str, body: Markup) -> Markup {
    maud::html! {
        (DOCTYPE)
        html lang="en" {
            head {
                script { (PreEscaped(THEME_PREPAINT)) }
                meta charset="UTF-8";
                meta name="viewport" content="width=device-width, initial-scale=1.0";
                title { (title) " - Theodore Tennant" }
                link rel="preconnect" href="https://fonts.googleapis.com";
                link rel="preconnect" href="https://fonts.gstatic.com" crossorigin;
                link href=(FONTS_HREF) rel="stylesheet";
                link rel="stylesheet" href="../styles/styles.css";
            }
            body {
                main class="container" {
                    header {
                        h1 { "Teddy Tennant" }
                        nav {
                            a href="/" { "← Back to Home" }
                        }
                    }
                    article class="essay" {
                        (body)
                    }
                }
                script src="../styles/theme.js" {}
            }
        }
    }
}

fn main() {
    let out_dir = std::env::args().nth(1).unwrap_or_else(|| "public/essays".to_string());
    let out = Path::new(&out_dir);
    fs::create_dir_all(out).expect("create output dir");

    for essay in essays::all() {
        let html = page(essay.title, essay.body).into_string();
        let path = out.join(format!("{}.html", essay.slug));
        fs::write(&path, html).unwrap_or_else(|e| panic!("write {}: {e}", path.display()));
        println!("essays-gen: wrote {}", path.display());
    }
}
