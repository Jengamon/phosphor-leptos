//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[cfg(any(feature = "finance", feature = "office"))]
#[component]
pub fn ChartScatter(
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
                <path d="M216,40H40A16,16,0,0,0,24,56V200a16,16,0,0,0,16,16H216a16,16,0,0,0,16-16V56A16,16,0,0,0,216,40ZM188,72a12,12,0,1,1-12,12A12,12,0,0,1,188,72Zm0,56a12,12,0,1,1-12,12A12,12,0,0,1,188,128Zm-40-16a12,12,0,1,1-12,12A12,12,0,0,1,148,112ZM124,72a12,12,0,1,1-12,12A12,12,0,0,1,124,72Zm-24,56a12,12,0,1,1-12,12A12,12,0,0,1,100,128Zm100,64H56a8,8,0,0,1-8-8V72a8,8,0,0,1,16,0V176H200a8,8,0,0,1,0,16Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path d="M224,64V208H32V48H208A16,16,0,0,1,224,64Z" opacity="0.2"></path>
    <path d="M232,208a8,8,0,0,1-8,8H32a8,8,0,0,1-8-8V48a8,8,0,0,1,16,0V200H224A8,8,0,0,1,232,208ZM132,160a12,12,0,1,0-12-12A12,12,0,0,0,132,160Zm-24-56A12,12,0,1,0,96,92,12,12,0,0,0,108,104ZM76,176a12,12,0,1,0-12-12A12,12,0,0,0,76,176Zm96-48a12,12,0,1,0-12-12A12,12,0,0,0,172,128Zm24-40a12,12,0,1,0-12-12A12,12,0,0,0,196,88Zm-20,76a12,12,0,1,0,12-12A12,12,0,0,0,176,164Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M228,208a4,4,0,0,1-4,4H32a4,4,0,0,1-4-4V48a4,4,0,0,1,8,0V204H224A4,4,0,0,1,228,208Zm-96-52a8,8,0,1,0-8-8A8,8,0,0,0,132,156Zm-24-56a8,8,0,1,0-8-8A8,8,0,0,0,108,100ZM76,172a8,8,0,1,0-8-8A8,8,0,0,0,76,172Zm96-48a8,8,0,1,0-8-8A8,8,0,0,0,172,124Zm24-40a8,8,0,1,0-8-8A8,8,0,0,0,196,84Zm-8,88a8,8,0,1,0-8-8A8,8,0,0,0,188,172Z"></path>
}.into_view(),
IconWeight::Bold => view! {
    <path d="M236,208a12,12,0,0,1-12,12H32a12,12,0,0,1-12-12V48a12,12,0,0,1,24,0V196H224A12,12,0,0,1,236,208ZM116,148a16,16,0,1,0,16-16A16,16,0,0,0,116,148Zm-8-40A16,16,0,1,0,92,92,16,16,0,0,0,108,108ZM76,180a16,16,0,1,0-16-16A16,16,0,0,0,76,180Zm96-48a16,16,0,1,0-16-16A16,16,0,0,0,172,132Zm24-40a16,16,0,1,0-16-16A16,16,0,0,0,196,92Zm-24,72a16,16,0,1,0,16-16A16,16,0,0,0,172,164Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M230,208a6,6,0,0,1-6,6H32a6,6,0,0,1-6-6V48a6,6,0,0,1,12,0V202H224A6,6,0,0,1,230,208Zm-98-50a10,10,0,1,0-10-10A10,10,0,0,0,132,158Zm-24-56A10,10,0,1,0,98,92,10,10,0,0,0,108,102ZM76,174a10,10,0,1,0-10-10A10,10,0,0,0,76,174Zm96-48a10,10,0,1,0-10-10A10,10,0,0,0,172,126Zm24-40a10,10,0,1,0-10-10A10,10,0,0,0,196,86Zm-8,68a10,10,0,1,0,10,10A10,10,0,0,0,188,154Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M232,208a8,8,0,0,1-8,8H32a8,8,0,0,1-8-8V48a8,8,0,0,1,16,0V200H224A8,8,0,0,1,232,208ZM132,160a12,12,0,1,0-12-12A12,12,0,0,0,132,160Zm-24-56A12,12,0,1,0,96,92,12,12,0,0,0,108,104ZM76,176a12,12,0,1,0-12-12A12,12,0,0,0,76,176Zm96-48a12,12,0,1,0-12-12A12,12,0,0,0,172,128Zm24-40a12,12,0,1,0-12-12A12,12,0,0,0,196,88Zm-20,76a12,12,0,1,0,12-12A12,12,0,0,0,176,164Z"></path>
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
