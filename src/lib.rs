use leptos::mount::mount_to_body;

mod components;
mod utils;

#[macro_use]
extern crate rust_i18n;

i18n!();

pub fn mount() {
    console_error_panic_hook::set_once();
    mount_to_body(components::app::App);
}
