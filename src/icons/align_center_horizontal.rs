//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[cfg(any(feature = "design", feature = "editor"))]
#[component]
pub fn AlignCenterHorizontal(
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
                <path d="M224,152v40a16,16,0,0,1-16,16H136v16a8,8,0,0,1-16,0V208H48a16,16,0,0,1-16-16V152a16,16,0,0,1,16-16h72V120H72a16,16,0,0,1-16-16V64A16,16,0,0,1,72,48h48V32a8,8,0,0,1,16,0V48h48a16,16,0,0,1,16,16v40a16,16,0,0,1-16,16H136v16h72A16,16,0,0,1,224,152Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path
        d="M64,104V64a8,8,0,0,1,8-8H184a8,8,0,0,1,8,8v40a8,8,0,0,1-8,8H72A8,8,0,0,1,64,104Zm144,40H48a8,8,0,0,0-8,8v40a8,8,0,0,0,8,8H208a8,8,0,0,0,8-8V152A8,8,0,0,0,208,144Z"
        opacity="0.2"
    ></path>
    <path d="M208,136H136V120h48a16,16,0,0,0,16-16V64a16,16,0,0,0-16-16H136V32a8,8,0,0,0-16,0V48H72A16,16,0,0,0,56,64v40a16,16,0,0,0,16,16h48v16H48a16,16,0,0,0-16,16v40a16,16,0,0,0,16,16h72v16a8,8,0,0,0,16,0V208h72a16,16,0,0,0,16-16V152A16,16,0,0,0,208,136ZM72,64H184v40H72ZM208,192H48V152H208v40Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M208,140H132V116h52a12,12,0,0,0,12-12V64a12,12,0,0,0-12-12H132V32a4,4,0,0,0-8,0V52H72A12,12,0,0,0,60,64v40a12,12,0,0,0,12,12h52v24H48a12,12,0,0,0-12,12v40a12,12,0,0,0,12,12h76v20a4,4,0,0,0,8,0V204h76a12,12,0,0,0,12-12V152A12,12,0,0,0,208,140ZM68,104V64a4,4,0,0,1,4-4H184a4,4,0,0,1,4,4v40a4,4,0,0,1-4,4H72A4,4,0,0,1,68,104Zm144,88a4,4,0,0,1-4,4H48a4,4,0,0,1-4-4V152a4,4,0,0,1,4-4H208a4,4,0,0,1,4,4Z"></path>
}.into_view(),
IconWeight::Bold => view! {
    <path d="M208,136H140V120h44a20,20,0,0,0,20-20V60a20,20,0,0,0-20-20H140V32a12,12,0,0,0-24,0v8H72A20,20,0,0,0,52,60v40a20,20,0,0,0,20,20h44v16H48a20,20,0,0,0-20,20v40a20,20,0,0,0,20,20h68v8a12,12,0,0,0,24,0v-8h68a20,20,0,0,0,20-20V156A20,20,0,0,0,208,136ZM76,64H180V96H76ZM204,192H52V160H204Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M208,138H134V118h50a14,14,0,0,0,14-14V64a14,14,0,0,0-14-14H134V32a6,6,0,0,0-12,0V50H72A14,14,0,0,0,58,64v40a14,14,0,0,0,14,14h50v20H48a14,14,0,0,0-14,14v40a14,14,0,0,0,14,14h74v18a6,6,0,0,0,12,0V206h74a14,14,0,0,0,14-14V152A14,14,0,0,0,208,138ZM70,104V64a2,2,0,0,1,2-2H184a2,2,0,0,1,2,2v40a2,2,0,0,1-2,2H72A2,2,0,0,1,70,104Zm140,88a2,2,0,0,1-2,2H48a2,2,0,0,1-2-2V152a2,2,0,0,1,2-2H208a2,2,0,0,1,2,2Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M208,136H136V120h48a16,16,0,0,0,16-16V64a16,16,0,0,0-16-16H136V32a8,8,0,0,0-16,0V48H72A16,16,0,0,0,56,64v40a16,16,0,0,0,16,16h48v16H48a16,16,0,0,0-16,16v40a16,16,0,0,0,16,16h72v16a8,8,0,0,0,16,0V208h72a16,16,0,0,0,16-16V152A16,16,0,0,0,208,136ZM72,64H184v40H72ZM208,192H48V152H208v40Z"></path>
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
