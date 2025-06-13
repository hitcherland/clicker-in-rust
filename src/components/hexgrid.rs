use crate::components::hexagon::Hexagon;
use crate::utils::hex2d::Hex2d;
use leptos::prelude::*;

#[component]
pub fn HexagonalGrid(#[prop(default = 2)] radius: i32) -> impl IntoView {
    let center = Hex2d {
        q: 0.0,
        r: 0.0,
        radius: 30.0,
    };

    let cells = move || center.range(radius);

    view! {
        <g>

            <For
                each=cells
                key=|p| p.to_string()
                let(cell)
            >
                <Hexagon hex=cell rotate_clockwise=true/>
            </For>
        </g>
    }
}
