//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[cfg(any(feature = "finance", feature = "development"))]
#[component]
pub fn SupersetProperOf(
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
                <path d="M208,32H48A16,16,0,0,0,32,48V208a16,16,0,0,0,16,16H208a16,16,0,0,0,16-16V48A16,16,0,0,0,208,32ZM128,192H80a8,8,0,0,1,0-16h48a48,48,0,0,0,0-96H80a8,8,0,0,1,0-16h48a64,64,0,0,1,0,128Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path d="M216,128a80,80,0,0,1-80,80H64V48h72A80,80,0,0,1,216,128Z" opacity="0.2"></path>
    <path d="M224,128a88.1,88.1,0,0,1-88,88H64a8,8,0,0,1,0-16h72a72,72,0,0,0,0-144H64a8,8,0,0,1,0-16h72A88.1,88.1,0,0,1,224,128Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M220,128a84.09,84.09,0,0,1-84,84H64a4,4,0,0,1,0-8h72a76,76,0,0,0,0-152H64a4,4,0,0,1,0-8h72A84.09,84.09,0,0,1,220,128Z"></path>
}.into_view(),
IconWeight::Bold => view! {
    <path d="M228,128a92.1,92.1,0,0,1-92,92H64a12,12,0,0,1,0-24h72a68,68,0,0,0,0-136H64a12,12,0,0,1,0-24h72A92.1,92.1,0,0,1,228,128Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M222,128a86.1,86.1,0,0,1-86,86H64a6,6,0,0,1,0-12h72a74,74,0,0,0,0-148H64a6,6,0,0,1,0-12h72A86.1,86.1,0,0,1,222,128Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M224,128a88.1,88.1,0,0,1-88,88H64a8,8,0,0,1,0-16h72a72,72,0,0,0,0-144H64a8,8,0,0,1,0-16h72A88.1,88.1,0,0,1,224,128Z"></path>
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
            id=move || id.get().map(|id| id.get())
            class=move || class.get().map(|cls| cls.get())
        >
            {body}
        </svg>
    }
}
