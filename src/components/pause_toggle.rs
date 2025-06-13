use icondata as i;
use leptos::prelude::*;
use leptos_icons::Icon;

turf::style_sheet!("src/scss/pause_toggle.scss");

#[component]
pub fn PauseToggle(pause: ReadSignal<bool>, pause_write: WriteSignal<bool>) -> impl IntoView {
    view! {
        <style>{STYLE_SHEET}</style>
        <div class=ClassName::PAUSE_TOGGLE on:click={move |_| pause_write.update(|n| *n = !(*n))}>
            <Show
                when=move || pause.get()
                fallback=|| view! {<Icon icon={i::FiPlay} />}
                >
                <Icon icon={i::FiPause} />
            </Show>
        </div>
    }
}
