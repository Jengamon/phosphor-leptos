//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[cfg(any(feature = "system", feature = "editor"))]
#[component]
pub fn Notches(
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
                <path d="M200,40V192a8,8,0,0,1-8,8H40a8,8,0,0,1-5.66-13.66l152-152A8,8,0,0,1,200,40Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path d="M192,40V192H40Z" opacity="0.2"></path>
    <path d="M195.06,32.61a8,8,0,0,0-8.72,1.73l-152,152A8,8,0,0,0,40,200H192a8,8,0,0,0,8-8V40A8,8,0,0,0,195.06,32.61ZM184,184H59.31L184,59.31Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M210.83,130.83l-80,80a4,4,0,1,1-5.66-5.66l80-80a4,4,0,1,1,5.66,5.66Zm-16-93.66a4,4,0,0,0-5.66,0l-152,152a4,4,0,0,0,5.66,5.66l152-152A4,4,0,0,0,194.83,37.17Z"></path>
}.into_view(),
IconWeight::Bold => view! {
    <path d="M216.49,136.49l-80,80a12,12,0,1,1-17-17l80-80a12,12,0,1,1,17,17Zm-16-105a12,12,0,0,0-17,0l-152,152a12,12,0,0,0,17,17l152-152A12,12,0,0,0,200.49,31.51Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M212.24,132.24l-80,80a6,6,0,1,1-8.48-8.48l80-80a6,6,0,1,1,8.48,8.48Zm-16-96.48a6,6,0,0,0-8.48,0l-152,152a6,6,0,1,0,8.48,8.48l152-152A6,6,0,0,0,196.24,35.76Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M213.66,133.66l-80,80a8,8,0,0,1-11.32-11.32l80-80a8,8,0,0,1,11.32,11.32Zm-16-99.32a8,8,0,0,0-11.32,0l-152,152a8,8,0,0,0,11.32,11.32l152-152A8,8,0,0,0,197.66,34.34Z"></path>
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
