//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[cfg(any(feature = "design", feature = "system"))]
#[component]
pub fn CardsThree(
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
                <path d="M224,104v96a16,16,0,0,1-16,16H48a16,16,0,0,1-16-16V104A16,16,0,0,1,48,88H208A16,16,0,0,1,224,104ZM56,72H200a8,8,0,0,0,0-16H56a8,8,0,0,0,0,16ZM72,40H184a8,8,0,0,0,0-16H72a8,8,0,0,0,0,16Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path
        d="M216,104v96a8,8,0,0,1-8,8H48a8,8,0,0,1-8-8V104a8,8,0,0,1,8-8H208A8,8,0,0,1,216,104Z"
        opacity="0.2"
    ></path>
    <path d="M208,88H48a16,16,0,0,0-16,16v96a16,16,0,0,0,16,16H208a16,16,0,0,0,16-16V104A16,16,0,0,0,208,88Zm0,112H48V104H208v96ZM48,64a8,8,0,0,1,8-8H200a8,8,0,0,1,0,16H56A8,8,0,0,1,48,64ZM64,32a8,8,0,0,1,8-8H184a8,8,0,0,1,0,16H72A8,8,0,0,1,64,32Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M208,92H48a12,12,0,0,0-12,12v96a12,12,0,0,0,12,12H208a12,12,0,0,0,12-12V104A12,12,0,0,0,208,92Zm4,108a4,4,0,0,1-4,4H48a4,4,0,0,1-4-4V104a4,4,0,0,1,4-4H208a4,4,0,0,1,4,4ZM52,64a4,4,0,0,1,4-4H200a4,4,0,0,1,0,8H56A4,4,0,0,1,52,64ZM68,32a4,4,0,0,1,4-4H184a4,4,0,0,1,0,8H72A4,4,0,0,1,68,32Z"></path>
}.into_view(),
IconWeight::Bold => view! {
    <path d="M208,96H48a20,20,0,0,0-20,20v84a20,20,0,0,0,20,20H208a20,20,0,0,0,20-20V116A20,20,0,0,0,208,96Zm-4,100H52V120H204ZM44,68A12,12,0,0,1,56,56H200a12,12,0,0,1,0,24H56A12,12,0,0,1,44,68ZM60,28A12,12,0,0,1,72,16H184a12,12,0,0,1,0,24H72A12,12,0,0,1,60,28Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M208,90H48a14,14,0,0,0-14,14v96a14,14,0,0,0,14,14H208a14,14,0,0,0,14-14V104A14,14,0,0,0,208,90Zm2,110a2,2,0,0,1-2,2H48a2,2,0,0,1-2-2V104a2,2,0,0,1,2-2H208a2,2,0,0,1,2,2ZM50,64a6,6,0,0,1,6-6H200a6,6,0,0,1,0,12H56A6,6,0,0,1,50,64ZM66,32a6,6,0,0,1,6-6H184a6,6,0,0,1,0,12H72A6,6,0,0,1,66,32Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M208,88H48a16,16,0,0,0-16,16v96a16,16,0,0,0,16,16H208a16,16,0,0,0,16-16V104A16,16,0,0,0,208,88Zm0,112H48V104H208v96ZM48,64a8,8,0,0,1,8-8H200a8,8,0,0,1,0,16H56A8,8,0,0,1,48,64ZM64,32a8,8,0,0,1,8-8H184a8,8,0,0,1,0,16H72A8,8,0,0,1,64,32Z"></path>
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
