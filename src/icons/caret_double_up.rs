//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[cfg(any(feature = "arrows"))]
#[component]
pub fn CaretDoubleUp(
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
                <path d="M213.66,194.34A8,8,0,0,1,208,208H48a8,8,0,0,1-5.66-13.66L108.69,128H48a8,8,0,0,1-5.66-13.66l80-80a8,8,0,0,1,11.32,0l80,80A8,8,0,0,1,208,128H147.31Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path d="M208,200H48l80-80Z" opacity="0.2"></path>
    <path d="M133.66,114.34a8,8,0,0,0-11.32,0l-80,80A8,8,0,0,0,48,208H208a8,8,0,0,0,5.66-13.66ZM67.31,192,128,131.31,188.69,192Zm-25-66.34a8,8,0,0,1,0-11.32l80-80a8,8,0,0,1,11.32,0l80,80a8,8,0,0,1-11.32,11.32L128,51.31,53.66,125.66A8,8,0,0,1,42.34,125.66Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M210.83,197.17a4,4,0,0,1-5.66,5.66L128,125.66,50.83,202.83a4,4,0,0,1-5.66-5.66l80-80a4,4,0,0,1,5.66,0Zm-160-74.34L128,45.66l77.17,77.17a4,4,0,1,0,5.66-5.66l-80-80a4,4,0,0,0-5.66,0l-80,80a4,4,0,0,0,5.66,5.66Z"></path>
}.into_view(),
IconWeight::Bold => view! {
    <path d="M216.49,191.51a12,12,0,0,1-17,17L128,137,56.49,208.49a12,12,0,0,1-17-17l80-80a12,12,0,0,1,17,0Zm-160-63L128,57l71.51,71.52a12,12,0,0,0,17-17l-80-80a12,12,0,0,0-17,0l-80,80a12,12,0,0,0,17,17Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M212.24,195.76a6,6,0,1,1-8.48,8.48L128,128.49,52.24,204.24a6,6,0,0,1-8.48-8.48l80-80a6,6,0,0,1,8.48,0Zm-160-71.52L128,48.49l75.76,75.75a6,6,0,0,0,8.48-8.48l-80-80a6,6,0,0,0-8.48,0l-80,80a6,6,0,0,0,8.48,8.48Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M213.66,194.34a8,8,0,0,1-11.32,11.32L128,131.31,53.66,205.66a8,8,0,0,1-11.32-11.32l80-80a8,8,0,0,1,11.32,0Zm-160-68.68L128,51.31l74.34,74.35a8,8,0,0,0,11.32-11.32l-80-80a8,8,0,0,0-11.32,0l-80,80a8,8,0,0,0,11.32,11.32Z"></path>
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
