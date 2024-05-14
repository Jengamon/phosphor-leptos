//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[cfg(any(feature = "commerce", feature = "objects"))]
#[component]
pub fn Sock(
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
                <path d="M192,16H104A16,16,0,0,0,88,32v76.69L49.25,147.43a58.92,58.92,0,0,0,83.32,83.32L201,162.34a23.85,23.85,0,0,0,7-17V32A16,16,0,0,0,192,16Zm0,16h0V56H104V32Zm-2.34,119L157.8,182.88a48,48,0,0,1,34.2-70.2v32.69A8,8,0,0,1,189.66,151Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path
        d="M200,112v33.37a16,16,0,0,1-4.69,11.32l-33,33A48,48,0,0,1,200,112Zm-8-88H104a8,8,0,0,0-8,8V56H200V32A8,8,0,0,0,192,24Z"
        opacity="0.2"
    ></path>
    <path d="M192,16H104A16,16,0,0,0,88,32v76.69L49.25,147.43a58.92,58.92,0,0,0,83.32,83.32L201,162.34a23.85,23.85,0,0,0,7-17V32A16,16,0,0,0,192,16Zm0,16h0V48H104V32ZM121.25,219.43a42.91,42.91,0,1,1-60.68-60.68l41.09-41.09A8,8,0,0,0,104,112V64h88v40.58A56.09,56.09,0,0,0,144,160a55.4,55.4,0,0,0,7.93,28.76ZM189.66,151l-25.91,25.91A39.6,39.6,0,0,1,160,160a40.05,40.05,0,0,1,32-39.19v24.56A8,8,0,0,1,189.66,151Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M192,20H104A12,12,0,0,0,92,32v78.34L52.08,150.26a54.91,54.91,0,0,0,77.66,77.66l68.4-68.41A19.85,19.85,0,0,0,204,145.37V32A12,12,0,0,0,192,20Zm-88,8h88a4,4,0,0,1,4,4V52H100V32A4,4,0,0,1,104,28Zm20.08,194.26a46.91,46.91,0,1,1-66.34-66.34l41.09-41.09A4,4,0,0,0,100,112V60h96v48.17A52.05,52.05,0,0,0,148,160a51.44,51.44,0,0,0,9,29.3Zm68.4-68.4-29.66,29.67A43.5,43.5,0,0,1,156,160a44.06,44.06,0,0,1,40-43.81v29.18A11.92,11.92,0,0,1,192.48,153.86Z"></path>
}.into_view(),
IconWeight::Bold => view! {
    <path d="M192,12H104A20,20,0,0,0,84,32v75L46.43,144.6a62.91,62.91,0,1,0,89,89l68.4-68.4a27.81,27.81,0,0,0,8.2-19.8V32A20,20,0,0,0,192,12Zm-4,24V52H108V36ZM118.43,216.6a38.91,38.91,0,1,1-55-55l41.09-41.08A12,12,0,0,0,108,112V76h80v25.21A60.09,60.09,0,0,0,140,160a59.37,59.37,0,0,0,7,28.07Zm68.4-68.4-21.51,21.51A36.06,36.06,0,0,1,188,126.06v19.31A4,4,0,0,1,186.83,148.2Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M192,18H104A14,14,0,0,0,90,32v77.51L50.67,148.85a56.91,56.91,0,1,0,80.48,80.48l68.41-68.4A21.88,21.88,0,0,0,206,145.37V32A14,14,0,0,0,192,18ZM104,30h88a2,2,0,0,1,2,2V50H102V32A2,2,0,0,1,104,30Zm18.67,190.85a44.92,44.92,0,0,1-63.52-63.52l41.09-41.09A6,6,0,0,0,102,112V62h92v44.34A54.07,54.07,0,0,0,146,160a53.39,53.39,0,0,0,8.47,29Zm68.4-68.41L163.22,180.3A41.54,41.54,0,0,1,158,160a42.05,42.05,0,0,1,36-41.56v26.93A9.93,9.93,0,0,1,191.07,152.44Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M192,16H104A16,16,0,0,0,88,32v76.69L49.25,147.43a58.92,58.92,0,0,0,83.32,83.32L201,162.34a23.85,23.85,0,0,0,7-17V32A16,16,0,0,0,192,16Zm0,16h0V48H104V32ZM121.25,219.43a42.91,42.91,0,1,1-60.68-60.68l41.09-41.09A8,8,0,0,0,104,112V64h88v40.58A56.09,56.09,0,0,0,144,160a55.4,55.4,0,0,0,7.93,28.76ZM189.66,151l-25.91,25.91A39.6,39.6,0,0,1,160,160a40.05,40.05,0,0,1,32-39.19v24.56A8,8,0,0,1,189.66,151Z"></path>
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
