//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[cfg(any(feature = "communication", feature = "nature"))]
#[component]
pub fn StarFour(
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
                <path d="M240,128a15.79,15.79,0,0,1-10.5,15l-63.44,23.07L143,229.5a16,16,0,0,1-30,0L89.94,166.06,26.5,143a16,16,0,0,1,0-30L89.94,89.94,113,26.5a16,16,0,0,1,30,0l23.07,63.44L229.5,113A15.79,15.79,0,0,1,240,128Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path
        d="M226.76,135.48l-66.94,24.34-24.34,66.94a8,8,0,0,1-15,0L96.18,159.82,29.24,135.48a8,8,0,0,1,0-15L96.18,96.18l24.34-66.94a8,8,0,0,1,15,0l24.34,66.94,66.94,24.34A8,8,0,0,1,226.76,135.48Z"
        opacity="0.2"
    ></path>
    <path d="M229.5,113,166.06,89.94,143,26.5a16,16,0,0,0-30,0L89.94,89.94,26.5,113a16,16,0,0,0,0,30l63.44,23.07L113,229.5a16,16,0,0,0,30,0l23.07-63.44L229.5,143a16,16,0,0,0,0-30ZM157.08,152.3a8,8,0,0,0-4.78,4.78L128,223.9l-24.3-66.82a8,8,0,0,0-4.78-4.78L32.1,128l66.82-24.3a8,8,0,0,0,4.78-4.78L128,32.1l24.3,66.82a8,8,0,0,0,4.78,4.78L223.9,128Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M228.13,116.77,162.94,93.06,139.23,27.87a11.95,11.95,0,0,0-22.46,0L93.06,93.06,27.87,116.77a11.95,11.95,0,0,0,0,22.46l65.19,23.71,23.71,65.19a11.95,11.95,0,0,0,22.46,0l23.71-65.19,65.19-23.71a11.95,11.95,0,0,0,0-22.46Zm-2.73,15-67,24.34a4,4,0,0,0-2.39,2.39l-24.34,67a4,4,0,0,1-7.44,0l-24.34-67a4,4,0,0,0-2.39-2.39L30.6,131.72a4,4,0,0,1,0-7.44L97.55,99.94a4,4,0,0,0,2.39-2.39L124.28,30.6a4,4,0,0,1,7.44,0l24.34,66.95a4,4,0,0,0,2.39,2.39l67,24.34a4,4,0,0,1,0,7.44Z"></path>
}.into_view(),
IconWeight::Bold => view! {
    <path d="M230.86,109.25,169.18,86.82,146.75,25.14a19.95,19.95,0,0,0-37.5,0L86.82,86.82,25.14,109.25a19.95,19.95,0,0,0,0,37.5l61.68,22.43,22.43,61.68a19.95,19.95,0,0,0,37.5,0l22.43-61.68,61.68-22.43a19.95,19.95,0,0,0,0-37.5Zm-75.14,39.29a12,12,0,0,0-7.18,7.18L128,212.21l-20.54-56.49a12,12,0,0,0-7.18-7.18L43.79,128l56.49-20.54a12,12,0,0,0,7.18-7.18L128,43.79l20.54,56.49a12,12,0,0,0,7.18,7.18L212.21,128Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M228.81,114.89,164.5,91.5,141.11,27.19a13.95,13.95,0,0,0-26.22,0L91.5,91.5,27.19,114.89a13.95,13.95,0,0,0,0,26.22L91.5,164.5l23.39,64.31a13.95,13.95,0,0,0,26.22,0L164.5,164.5l64.31-23.39a13.95,13.95,0,0,0,0-26.22Zm-4.1,15-66.94,24.34a6,6,0,0,0-3.59,3.59l-24.34,66.94a2,2,0,0,1-3.68,0l-24.34-66.94a6,6,0,0,0-3.59-3.59L31.29,129.84a2,2,0,0,1,0-3.68l66.94-24.34a6,6,0,0,0,3.59-3.59l24.34-66.94a2,2,0,0,1,3.68,0l24.34,66.94a6,6,0,0,0,3.59,3.59l66.94,24.34a2,2,0,0,1,0,3.68Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M229.5,113,166.06,89.94,143,26.5a16,16,0,0,0-30,0L89.94,89.94,26.5,113a16,16,0,0,0,0,30l63.44,23.07L113,229.5a16,16,0,0,0,30,0l23.07-63.44L229.5,143a16,16,0,0,0,0-30ZM157.08,152.3a8,8,0,0,0-4.78,4.78L128,223.9l-24.3-66.82a8,8,0,0,0-4.78-4.78L32.1,128l66.82-24.3a8,8,0,0,0,4.78-4.78L128,32.1l24.3,66.82a8,8,0,0,0,4.78,4.78L223.9,128Z"></path>
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
