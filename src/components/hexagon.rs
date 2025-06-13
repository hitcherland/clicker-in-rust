use crate::utils::hex2d::Hex2d;
use crate::utils::vec2d::Vec2d;
use leptos::prelude::*;
use std::f64::consts::PI;

struct Hxdata { r: f64, transform: String }

#[component]
pub fn Hexagon(
    #[prop(default=Hex2d{q:0.0, r:0.0, radius: 30.0})] hex: Hex2d,
    #[prop(default = false)] rotate_clockwise: bool,
) -> impl IntoView {
    let points = move || {
        std::array::from_fn::<_, 6, _>(|i| {
            let angle = 2.0 * PI * (i as f64) / 6.0;
            return Vec2d {
                x: hex.radius * angle.sin(),
                y: hex.radius * angle.cos(),
            }
            .to_string();
        })
        .join(",")
    };

    let inner = move || {
        let tri: Vec<String> = vec![
            Vec2d { x: 3f64.sqrt() * 0.4 * hex.radius, y: 0.0}.to_string(),
            Vec2d { x: 0.0, y: 0.1 * hex.radius}.to_string(),
            Vec2d { x: 0.0, y: -0.1 * hex.radius}.to_string(),
        ];

        return tri.join(",");
    };

    let values = move || {
        if rotate_clockwise {
            return "0;-60;-120;-180;-240;-300;-360";
        }
        return "0;60;120;180;240;300;360";
    };

    let hxdata = move || {
        return Hxdata {
            r: 0.15 * hex.radius,
            transform: format!("translate({}, {})", hex.x(), hex.y()),
        };
    };

    view! {
        <g transform={hxdata().transform}>
            <polygon points=points stroke-width=2 stroke="white"/>
            <polygon points=inner fill="white">
                <animateTransform
                    attributeName="transform"
                    attributeType="XML"
                    type="rotate"
                    dur="6s"
                    calcMode="spline"
                    values=values
                    keySplines="0.99 0.1 0.1 0.99;0.99 0.1 0.1 0.99;0.99 0.1 0.1 0.99;0.99 0.1 0.1 0.99;0.99 0.1 0.1 0.99;0.99 0.1 0.1 0.99"
                    repeatCount="indefinite"
                />
            </polygon>
            <circle r={hxdata().r} fill="white"/>
        </g>
    }
}
