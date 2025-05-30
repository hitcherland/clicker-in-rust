use leptos::prelude::*;

turf::style_sheet!("src/scss/main.scss");

#[component]
pub fn App() -> impl IntoView {
    let (count, click_count) = signal(0);

    view! {
        <style>{STYLE_SHEET}</style>
        <button on:click=move |_| click_count.update(|n| *n += 1)>"Click me: " {count}</button>
        <p>"Double count: " {move || count.get() * 2}</p>
    }
}