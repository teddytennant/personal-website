mod app;
mod bg;
mod components;
mod content;
mod format;
mod lenis;
mod media;
mod motion;
mod pages;
mod theme;

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(app::App);
}
