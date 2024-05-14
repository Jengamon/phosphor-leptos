//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[cfg(any(feature = "design", feature = "editor"))]
#[component]
pub fn TextHFour(
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
                <path d="M164.46,144,184,119.13V144ZM224,48V208a16,16,0,0,1-16,16H48a16,16,0,0,1-16-16V48A16,16,0,0,1,48,32H208A16,16,0,0,1,224,48ZM128,80a8,8,0,0,0-16,0v32H72V80a8,8,0,0,0-16,0v80a8,8,0,0,0,16,0V128h40v32a8,8,0,0,0,16,0Zm84,72a8,8,0,0,0-8-8h-4V96a8,8,0,0,0-14.29-4.94l-44,56A8,8,0,0,0,148,160h36v16a8,8,0,0,0,16,0V160h4A8,8,0,0,0,212,152Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path
        d="M248,72V192a16,16,0,0,1-16,16H56a16,16,0,0,1-16-16V56H232A16,16,0,0,1,248,72Z"
        opacity="0.2"
    ></path>
    <path d="M152,56V176a8,8,0,0,1-16,0V124H48v52a8,8,0,0,1-16,0V56a8,8,0,0,1,16,0v52h88V56a8,8,0,0,1,16,0ZM256,184a8,8,0,0,1-8,8h-8v16a8,8,0,0,1-16,0V192H176a8,8,0,0,1-6.31-12.91l56-72A8,8,0,0,1,240,112v64h8A8,8,0,0,1,256,184Zm-32-48.68L192.36,176H224Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M148,56V176a4,4,0,0,1-8,0V120H44v56a4,4,0,0,1-8,0V56a4,4,0,0,1,8,0v56h96V56a4,4,0,0,1,8,0ZM252,184a4,4,0,0,1-4,4H236v20a4,4,0,0,1-8,0V188H176a4,4,0,0,1-3.16-6.46l56-72A4,4,0,0,1,236,112v68h12A4,4,0,0,1,252,184Zm-24-60.34L184.18,180H228Z"></path>
}.into_view(),
IconWeight::Bold => view! {
    <path d="M156,56V176a12,12,0,0,1-24,0V128H52v48a12,12,0,0,1-24,0V56a12,12,0,0,1,24,0v48h80V56a12,12,0,0,1,24,0ZM256,184a12,12,0,0,1-12,12v12a12,12,0,0,1-24,0V196H180a12,12,0,0,1-9.73-19l52-72A12,12,0,0,1,244,112v60A12,12,0,0,1,256,184Zm-36-34.89L203.47,172H220Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M150,56V176a6,6,0,0,1-12,0V122H46v54a6,6,0,0,1-12,0V56a6,6,0,0,1,12,0v54h92V56a6,6,0,0,1,12,0ZM254,184a6,6,0,0,1-6,6H238v18a6,6,0,0,1-12,0V190H176a6,6,0,0,1-4.74-9.68l56-72A6,6,0,0,1,238,112v66h10A6,6,0,0,1,254,184Zm-28-54.51L188.27,178H226Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M152,56V176a8,8,0,0,1-16,0V124H48v52a8,8,0,0,1-16,0V56a8,8,0,0,1,16,0v52h88V56a8,8,0,0,1,16,0ZM256,184a8,8,0,0,1-8,8h-8v16a8,8,0,0,1-16,0V192H176a8,8,0,0,1-6.31-12.91l56-72A8,8,0,0,1,240,112v64h8A8,8,0,0,1,256,184Zm-32-48.68L192.36,176H224Z"></path>
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
