use leptos::leptos_dom::helpers::{document, request_animation_frame};
use leptos::prelude::*;
use leptos_meta::Title;

turf::style_sheet!("src/scss/main.scss");

use crate::components::lifespan::Lifespan;
use crate::utils::stats::stats;

struct Animator {
    pub paused: ReadSignal<bool>,
    pub active_time_ms: WriteSignal<f64>,
    pub last_time_ms: f64,
}

impl Animator {
    pub fn clone(&self) -> Animator {
        return Animator {
            paused: self.paused,
            active_time_ms: self.active_time_ms,
            last_time_ms: self.last_time_ms,
        };
    }

    pub fn call(&mut self) {
        let time = document()
            .timeline()
            .current_time()
            .expect("Failed to get current time");
        if !self.paused.get_untracked() {
            self.active_time_ms
                .update(|n| *n += time - self.last_time_ms);
        }
        self.last_time_ms = time;
        let mut clone = self.clone();
        request_animation_frame(move || clone.call());
    }
}

#[component]
pub fn App() -> impl IntoView {
    leptos_meta::provide_meta_context();
    let stats = stats();

    let time = document()
        .timeline()
        .current_time()
        .expect("Failed to get current time");

    Animator {
        last_time_ms: time,
        paused: stats.paused.read,
        active_time_ms: stats.active_time_ms.write,
    }
    .call();

    view! {
        <style>{STYLE_SHEET}</style>
        <div>
            <Title text=move || {t!{"title"}} />
            <Lifespan
                max_lifespan_gamedays=stats.max_lifespan_gamedays.read
                active_time_ms=stats.active_time_ms.read
                gamedays_per_ms=stats.gamedays_per_ms.read
                pause=stats.paused.read
                pause_write=stats.paused.write
            />
        </div>
    }
}
