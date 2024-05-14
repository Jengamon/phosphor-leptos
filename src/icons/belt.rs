//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[cfg(any(feature = "commerce", feature = "objects"))]
#[component]
pub fn Belt(
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
                <path d="M64,80v96a8,8,0,0,1-16,0H8a8,8,0,0,1-8-8V88a8,8,0,0,1,8-8H48a8,8,0,0,1,16,0Zm192,8v80a8,8,0,0,1-8,8H189.83A16,16,0,0,1,176,184H112a16,16,0,0,1-13.83-8H84a4,4,0,0,1-4-4V84a4,4,0,0,1,4-4H98.17A16,16,0,0,1,112,72h64a16,16,0,0,1,13.83,8H248A8,8,0,0,1,256,88Zm-80,79.8V136H144a8,8,0,0,1,0-16h32V88H112v80h64C176,167.93,176,167.87,176,167.8Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path d="M8,88h96v80H8Zm176,0v80h64V88Z" opacity="0.2"></path>
    <path d="M248,160H192V96h56a8,8,0,0,0,0-16H189.84A16,16,0,0,0,176,72H112a16,16,0,0,0-13.84,8H64a8,8,0,0,0-16,0H8A8,8,0,0,0,8,96H48v64H8a8,8,0,0,0,0,16H48a8,8,0,0,0,16,0H98.16A16,16,0,0,0,112,184h64a16,16,0,0,0,13.84-8H248a8,8,0,0,0,0-16ZM64,96H96v64H64Zm48,72V88h64v32H144a8,8,0,0,0,0,16h32v31.8c0,.07,0,.13,0,.2Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M248,164H188V92h60a4,4,0,0,0,0-8H187.3A12,12,0,0,0,176,76H112a12,12,0,0,0-11.3,8H60V80a4,4,0,0,0-8,0v4H8a4,4,0,0,0,0,8H52v72H8a4,4,0,0,0,0,8H52v4a4,4,0,0,0,8,0v-4h40.7a12,12,0,0,0,11.3,8h64a12,12,0,0,0,11.3-8H248a4,4,0,0,0,0-8ZM60,92h40v72H60Zm116,80H112a4,4,0,0,1-4-4V88a4,4,0,0,1,4-4h64a4,4,0,0,1,4,4v36H144a4,4,0,0,0,0,8h36v36A4,4,0,0,1,176,172Z"></path>
}.into_view(),
IconWeight::Bold => view! {
    <path d="M244,156H196V100h48a12,12,0,0,0,0-24H192a20,20,0,0,0-16-8H112a20,20,0,0,0-16,8H67.3a12,12,0,0,0-22.6,0H12a12,12,0,0,0,0,24H44v56H12a12,12,0,0,0,0,24H44.7a12,12,0,0,0,22.6,0H96a20,20,0,0,0,16,8h64a20,20,0,0,0,16-8h52a12,12,0,0,0,0-24ZM68,100H92v56H68Zm48-8h56v24H144a12,12,0,0,0,0,24h28v24H116Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M248,162H190V94h58a6,6,0,0,0,0-12H188.63A14,14,0,0,0,176,74H112a14,14,0,0,0-12.63,8H62V80a6,6,0,0,0-12,0v2H8A6,6,0,0,0,8,94H50v68H8a6,6,0,0,0,0,12H50v2a6,6,0,0,0,12,0v-2H99.37A14,14,0,0,0,112,182h64a14,14,0,0,0,12.63-8H248a6,6,0,0,0,0-12ZM62,94H98v68H62Zm114,76H112a2,2,0,0,1-2-2V88a2,2,0,0,1,2-2h64a2,2,0,0,1,2,2v34H144a6,6,0,0,0,0,12h34v34A2,2,0,0,1,176,170Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M248,160H192V96h56a8,8,0,0,0,0-16H189.83A16,16,0,0,0,176,72H112a16,16,0,0,0-13.83,8H64a8,8,0,0,0-16,0H8A8,8,0,0,0,8,96H48v64H8a8,8,0,0,0,0,16H48a8,8,0,0,0,16,0H98.17A16,16,0,0,0,112,184h64a16,16,0,0,0,13.83-8H248a8,8,0,0,0,0-16ZM64,96H96v64H64Zm48,72V88h64v32H144a8,8,0,0,0,0,16h32v31.8c0,.07,0,.13,0,.2Z"></path>
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
