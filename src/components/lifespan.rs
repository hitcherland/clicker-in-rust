use crate::components::pause_toggle::PauseToggle;
use crate::components::progress_bar::ProgressBar;
use leptos::prelude::*;
use rust_i18n::t;

#[component]
pub fn Lifespan(
    max_lifespan_gamedays: ReadSignal<f64>,
    gamedays_per_ms: ReadSignal<f64>,
    active_time_ms: ReadSignal<f64>,
    pause: ReadSignal<bool>,
    pause_write: WriteSignal<bool>,
) -> impl IntoView {
    let value = move || active_time_ms.get() * gamedays_per_ms.get();
    let label = t!("lifespan").into_owned();

    view! {
        <ProgressBar
          label=label
          value=Signal::derive(value)
          max=max_lifespan_gamedays
          paused=pause
        >
            <PauseToggle pause=pause pause_write=pause_write/>
        </ProgressBar>
    }
}
