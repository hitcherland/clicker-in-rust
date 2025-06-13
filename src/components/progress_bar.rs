use leptos::prelude::*;

turf::style_sheet!("src/scss/progress_bar.scss");

#[component]
pub fn ProgressBar(
    #[prop(into)] label: Signal<String>,
    #[prop(into)] value: Signal<f64>,
    max: ReadSignal<f64>,
    children: Children,
) -> impl IntoView {
    view! {
        <style>{STYLE_SHEET}</style>
        <div class=ClassName::PROGRESS_BAR>
            <label>
                <span>{label}</span>
                <progress value=value max=max>{value}</progress>
                {children()}
            </label>
        </div>
    }
}
