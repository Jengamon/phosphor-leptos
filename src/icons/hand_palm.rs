//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[cfg(any(feature = "system", feature = "people"))]
#[component]
pub fn HandPalm(
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
                <path d="M216,104v48a88,88,0,0,1-176,0V64a16,16,0,0,1,32,0v56a8,8,0,0,0,16,0V32a16,16,0,0,1,32,0v80a8,8,0,0,0,16,0V48a16,16,0,0,1,32,0v80.67A48.08,48.08,0,0,0,128,176a8,8,0,0,0,16,0,32,32,0,0,1,32-32,8,8,0,0,0,8-8V104a16,16,0,0,1,32,0Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path
        d="M208,116v36a80,80,0,0,1-160,0V76a20,20,0,0,1,40,0V44a20,20,0,0,1,40,0V60a20,20,0,0,1,40,0v56a20,20,0,0,1,40,0Z"
        opacity="0.2"
    ></path>
    <path d="M188,88a27.75,27.75,0,0,0-12,2.71V60a28,28,0,0,0-41.36-24.6A28,28,0,0,0,80,44v6.71A27.75,27.75,0,0,0,68,48,28,28,0,0,0,40,76v76a88,88,0,0,0,176,0V116A28,28,0,0,0,188,88Zm12,64a72,72,0,0,1-144,0V76a12,12,0,0,1,24,0v44a8,8,0,0,0,16,0V44a12,12,0,0,1,24,0v68a8,8,0,0,0,16,0V60a12,12,0,0,1,24,0v68.67A48.08,48.08,0,0,0,120,176a8,8,0,0,0,16,0,32,32,0,0,1,32-32,8,8,0,0,0,8-8V116a12,12,0,0,1,24,0Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M188,92a23.9,23.9,0,0,0-16,6.13V60a24,24,0,0,0-40.07-17.8A24,24,0,0,0,84,44V58.13A24,24,0,0,0,44,76v76a84,84,0,0,0,168,0V116A24,24,0,0,0,188,92Zm16,60a76,76,0,0,1-152,0V76a16,16,0,0,1,32,0v44a4,4,0,0,0,8,0V44a16,16,0,0,1,32,0v68a4,4,0,0,0,8,0V60a16,16,0,0,1,32,0v72.18A44.06,44.06,0,0,0,124,176a4,4,0,0,0,8,0,36,36,0,0,1,36-36,4,4,0,0,0,4-4V116a16,16,0,0,1,32,0Z"></path>
}.into_view(),
IconWeight::Bold => view! {
    <path d="M188,84a32,32,0,0,0-8,1V60a32,32,0,0,0-43.21-30A32,32,0,0,0,76,44v1A32,32,0,0,0,36,76v76a92,92,0,0,0,184,0V116A32,32,0,0,0,188,84Zm8,68a68,68,0,0,1-136,0V76a8,8,0,0,1,16,0v40a12,12,0,0,0,24,0V44a8,8,0,0,1,16,0v64a12,12,0,0,0,24,0V60a8,8,0,0,1,16,0v65.4A52.09,52.09,0,0,0,116,176a12,12,0,0,0,24,0,28,28,0,0,1,28-28,12,12,0,0,0,12-12V116a8,8,0,0,1,16,0Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M188,90a25.8,25.8,0,0,0-14,4.11V60a26,26,0,0,0-40.59-21.51A26,26,0,0,0,82,44V54.11A26,26,0,0,0,42,76v76a86,86,0,0,0,172,0V116A26,26,0,0,0,188,90Zm14,62a74,74,0,0,1-148,0V76a14,14,0,0,1,28,0v44a6,6,0,0,0,12,0V44a14,14,0,0,1,28,0v68a6,6,0,0,0,12,0V60a14,14,0,0,1,28,0v70.39A46.07,46.07,0,0,0,122,176a6,6,0,0,0,12,0,34,34,0,0,1,34-34,6,6,0,0,0,6-6V116a14,14,0,0,1,28,0Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M188,88a27.75,27.75,0,0,0-12,2.71V60a28,28,0,0,0-41.36-24.6A28,28,0,0,0,80,44v6.71A27.75,27.75,0,0,0,68,48,28,28,0,0,0,40,76v76a88,88,0,0,0,176,0V116A28,28,0,0,0,188,88Zm12,64a72,72,0,0,1-144,0V76a12,12,0,0,1,24,0v44a8,8,0,0,0,16,0V44a12,12,0,0,1,24,0v68a8,8,0,0,0,16,0V60a12,12,0,0,1,24,0v68.67A48.08,48.08,0,0,0,120,176a8,8,0,0,0,16,0,32,32,0,0,1,32-32,8,8,0,0,0,8-8V116a12,12,0,0,1,24,0Z"></path>
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
