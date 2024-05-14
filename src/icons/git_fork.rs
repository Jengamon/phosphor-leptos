//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[cfg(any(feature = "development"))]
#[component]
pub fn GitFork(
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
                <path d="M224,64a32,32,0,1,0-40,31v17a8,8,0,0,1-8,8H80a8,8,0,0,1-8-8V95a32,32,0,1,0-16,0v17a24,24,0,0,0,24,24h40v25a32,32,0,1,0,16,0V136h40a24,24,0,0,0,24-24V95A32.06,32.06,0,0,0,224,64ZM144,192a16,16,0,1,1-16-16A16,16,0,0,1,144,192Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path
        d="M88,64A24,24,0,1,1,64,40,24,24,0,0,1,88,64ZM192,40a24,24,0,1,0,24,24A24,24,0,0,0,192,40Z"
        opacity="0.2"
    ></path>
    <path d="M224,64a32,32,0,1,0-40,31v17a8,8,0,0,1-8,8H80a8,8,0,0,1-8-8V95a32,32,0,1,0-16,0v17a24,24,0,0,0,24,24h40v25a32,32,0,1,0,16,0V136h40a24,24,0,0,0,24-24V95A32.06,32.06,0,0,0,224,64ZM48,64A16,16,0,1,1,64,80,16,16,0,0,1,48,64Zm96,128a16,16,0,1,1-16-16A16,16,0,0,1,144,192ZM192,80a16,16,0,1,1,16-16A16,16,0,0,1,192,80Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M220,64a28,28,0,1,0-32,27.71V112a12,12,0,0,1-12,12H80a12,12,0,0,1-12-12V91.71a28,28,0,1,0-8,0V112a20,20,0,0,0,20,20h44v32.29a28,28,0,1,0,8,0V132h44a20,20,0,0,0,20-20V91.71A28,28,0,0,0,220,64ZM44,64A20,20,0,1,1,64,84,20,20,0,0,1,44,64ZM148,192a20,20,0,1,1-20-20A20,20,0,0,1,148,192ZM192,84a20,20,0,1,1,20-20A20,20,0,0,1,192,84Z"></path>
}.into_view(),
IconWeight::Bold => view! {
    <path d="M228,64a36,36,0,1,0-48,33.94V112a4,4,0,0,1-4,4H80a4,4,0,0,1-4-4V97.94a36,36,0,1,0-24,0V112a28,28,0,0,0,28,28h36v18.06a36,36,0,1,0,24,0V140h36a28,28,0,0,0,28-28V97.94A36.07,36.07,0,0,0,228,64ZM64,52A12,12,0,1,1,52,64,12,12,0,0,1,64,52Zm64,152a12,12,0,1,1,12-12A12,12,0,0,1,128,204ZM192,76a12,12,0,1,1,12-12A12,12,0,0,1,192,76Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M222,64a30,30,0,1,0-36,29.4V112a10,10,0,0,1-10,10H80a10,10,0,0,1-10-10V93.4a30,30,0,1,0-12,0V112a22,22,0,0,0,22,22h42v28.6a30,30,0,1,0,12,0V134h42a22,22,0,0,0,22-22V93.4A30.05,30.05,0,0,0,222,64ZM46,64A18,18,0,1,1,64,82,18,18,0,0,1,46,64ZM146,192a18,18,0,1,1-18-18A18,18,0,0,1,146,192ZM192,82a18,18,0,1,1,18-18A18,18,0,0,1,192,82Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M224,64a32,32,0,1,0-40,31v17a8,8,0,0,1-8,8H80a8,8,0,0,1-8-8V95a32,32,0,1,0-16,0v17a24,24,0,0,0,24,24h40v25a32,32,0,1,0,16,0V136h40a24,24,0,0,0,24-24V95A32.06,32.06,0,0,0,224,64ZM48,64A16,16,0,1,1,64,80,16,16,0,0,1,48,64Zm96,128a16,16,0,1,1-16-16A16,16,0,0,1,144,192ZM192,80a16,16,0,1,1,16-16A16,16,0,0,1,192,80Z"></path>
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
