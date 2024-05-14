//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[cfg(any(feature = "commerce", feature = "map"))]
#[component]
pub fn Bread(
    #[prop(into, default = MaybeSignal::Static(IconWeight::Regular))] weight: MaybeSignal<
        IconWeight,
    >,
    #[prop(into, default = TextProp::from("1em"))] size: TextProp,
    #[prop(into, default = TextProp::from("currentColor"))] color: TextProp,
    #[prop(into, default = MaybeSignal::Static(false))] mirrored: MaybeSignal<bool>,
    #[prop(into, optional)] id: MaybeProp<TextProp>,
    #[prop(into, optional)] class: MaybeProp<TextProp>,
) -> impl IntoView {
    let body = Signal::derive(move || {
        match weight.get() {
            IconWeight::Fill => view! {
                <path d="M200,40H48a40,40,0,0,0-16,76.65V200a16,16,0,0,0,16,16H200a16,16,0,0,0,16-16V116.65A40,40,0,0,0,200,40Zm-56,64a8,8,0,0,0,0,16v80H48V120a8,8,0,0,0,0-16,24,24,0,0,1,0-48h96a24,24,0,0,1,0,48Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path
        d="M208,111v89a8,8,0,0,1-8,8H144a8,8,0,0,0,8-8V111a32,32,0,0,0-8-63h56a32,32,0,0,1,8,63Z"
        opacity="0.2"
    ></path>
    <path d="M240,80a40,40,0,0,0-40-40H48a40,40,0,0,0-16,76.65V200a16,16,0,0,0,16,16H200a16,16,0,0,0,16-16V116.65A40.06,40.06,0,0,0,240,80ZM48,120a8,8,0,0,0,0-16,24,24,0,0,1,0-48h96a24,24,0,0,1,0,48,8,8,0,0,0,0,16v80H48Zm152-16a8,8,0,0,0,0,16v80H160V116.65A40,40,0,0,0,176,56h24a24,24,0,0,1,0,48Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M236,80a36,36,0,0,0-36-36H48a36,36,0,0,0-12,69.94V200a12,12,0,0,0,12,12H200a12,12,0,0,0,12-12V113.94A36.07,36.07,0,0,0,236,80ZM44,200V115.77a35.22,35.22,0,0,0,4,.23,4,4,0,0,0,0-8,28,28,0,0,1,0-56h96a28,28,0,0,1,0,56,4,4,0,0,0,0,8,35.22,35.22,0,0,0,4-.23V200a4,4,0,0,1-4,4H48A4,4,0,0,1,44,200Zm156-92a4,4,0,0,0,0,8,35.22,35.22,0,0,0,4-.23V200a4,4,0,0,1-4,4H155.3a11.8,11.8,0,0,0,.7-4V113.94A36,36,0,0,0,166.59,52H200a28,28,0,0,1,0,56Z"></path>
}.into_view(),
IconWeight::Bold => view! {
    <path d="M244,80a44.05,44.05,0,0,0-44-44H48a44,44,0,0,0-20,83.17V200a20,20,0,0,0,20,20H200a20,20,0,0,0,20-20V119.17A44,44,0,0,0,244,80ZM52,123.3A12,12,0,0,0,48,100a20,20,0,0,1,0-40h92a20,20,0,0,1,0,40,12,12,0,0,0-4,23.3V196H52ZM200,100a12,12,0,0,0-4,23.3V196H160V119.17A43.92,43.92,0,0,0,179.17,60H200a20,20,0,0,1,0,40Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M238,80a38,38,0,0,0-38-38H48a38,38,0,0,0-14,73.32V200a14,14,0,0,0,14,14H200a14,14,0,0,0,14-14V115.32A38.07,38.07,0,0,0,238,80ZM46,200V118c.66,0,1.33,0,2,0a6,6,0,0,0,0-12,26,26,0,0,1,0-52h96a26,26,0,0,1,0,52,6,6,0,0,0,0,12c.67,0,1.34,0,2,0V200a2,2,0,0,1-2,2H48A2,2,0,0,1,46,200Zm154-94a6,6,0,0,0,0,12c.67,0,1.34,0,2,0V200a2,2,0,0,1-2,2H157.84a14.71,14.71,0,0,0,.16-2V115.32A38,38,0,0,0,171.68,54H200a26,26,0,0,1,0,52Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M240,80a40,40,0,0,0-40-40H48a40,40,0,0,0-16,76.65V200a16,16,0,0,0,16,16H200a16,16,0,0,0,16-16V116.65A40.06,40.06,0,0,0,240,80ZM48,120a8,8,0,0,0,0-16,24,24,0,0,1,0-48h96a24,24,0,0,1,0,48,8,8,0,0,0,0,16v80H48Zm152-16a8,8,0,0,0,0,16v80H160V116.65A40,40,0,0,0,176,56h24a24,24,0,0,1,0,48Z"></path>
}.into_view()
        }
    });

    let transform = move || if mirrored.get() { "scale(-1, 1)" } else { "" };
    let height = size.clone();

    view! {
        <svg
            xmlns="http://www.w3.org/2000/svg"
            width=move || size.get()
            height=move || height.get()
            fill=color
            transform=transform
            viewBox="0 0 256 256"
            id=move || id.get().unwrap_or(TextProp::from(""))
            class=move || class.get().unwrap_or(TextProp::from(""))
        >
            {body}
        </svg>
    }
}
