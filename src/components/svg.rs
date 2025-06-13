use leptos::prelude::*;

use crate::components::hexgrid::HexagonalGrid;

#[component]
pub fn SVG() -> impl IntoView {
    view! {
        <svg width="500" height="500">
            <g transform="translate(250, 250)">
                <HexagonalGrid/>
            </g>
        </svg>
    }
}
