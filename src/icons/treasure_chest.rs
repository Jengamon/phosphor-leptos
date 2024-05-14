//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[cfg(any(feature = "games", feature = "objects"))]
#[component]
pub fn TreasureChest(
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
                <path d="M240,124v68a16,16,0,0,1-16,16H32a16,16,0,0,1-16-16V124a4,4,0,0,1,4-4H56v64a8,8,0,0,0,8.53,8A8.17,8.17,0,0,0,72,183.73V120h40v20a4,4,0,0,0,4,4h24a4,4,0,0,0,4-4V120h40v64a8,8,0,0,0,8.53,8,8.17,8.17,0,0,0,7.47-8.25V120h36A4,4,0,0,1,240,124ZM184,40H72A56,56,0,0,0,16,96v4a4,4,0,0,0,4,4H56V64.27A8.17,8.17,0,0,1,63.47,56,8,8,0,0,1,72,64v40h40V92a4,4,0,0,1,4-4h24a4,4,0,0,1,4,4v12h40V64.27A8.17,8.17,0,0,1,191.47,56,8,8,0,0,1,200,64v40h36a4,4,0,0,0,4-4V96A56,56,0,0,0,184,40Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path d="M232,112v80a8,8,0,0,1-8,8H32a8,8,0,0,1-8-8V112h88v32h32V112Z" opacity="0.2"></path>
    <path d="M184,40H72A56.06,56.06,0,0,0,16,96v96a16,16,0,0,0,16,16H224a16,16,0,0,0,16-16V96A56.06,56.06,0,0,0,184,40Zm40,56v8H192V56.8A40.07,40.07,0,0,1,224,96Zm-88,40H120V104h16Zm-24,16h32a8,8,0,0,0,8-8V120h24v72H80V120h24v24A8,8,0,0,0,112,152Zm40-48V96a8,8,0,0,0-8-8H112a8,8,0,0,0-8,8v8H80V56h96v48ZM64,56.8V104H32V96A40.07,40.07,0,0,1,64,56.8ZM32,120H64v72H32Zm192,72H192V120h32v72Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M184,44H72A52.06,52.06,0,0,0,20,96v96a12,12,0,0,0,12,12H224a12,12,0,0,0,12-12V96A52.06,52.06,0,0,0,184,44Zm44,52v12H188V52.19A44.06,44.06,0,0,1,228,96Zm-88,44H116V100h24Zm-28,8h32a4,4,0,0,0,4-4V116h32v80H76V116h32v28A4,4,0,0,0,112,148Zm36-40V96a4,4,0,0,0-4-4H112a4,4,0,0,0-4,4v12H76V52H180v56ZM68,52.19V108H28V96A44.06,44.06,0,0,1,68,52.19ZM28,192V116H68v80H32A4,4,0,0,1,28,192Zm196,4H188V116h40v76A4,4,0,0,1,224,196Z"></path>
}.into_view(),
IconWeight::Bold => view! {
    <path d="M184,36H72A60.07,60.07,0,0,0,12,96v96a20,20,0,0,0,20,20H224a20,20,0,0,0,20-20V96A60.07,60.07,0,0,0,184,36Zm36,60v4H192V60.91A36.05,36.05,0,0,1,220,96Zm-92,60a12,12,0,0,0,12-12V124h28v64H88V124h28v20A12,12,0,0,0,128,156Zm12-56V96a12,12,0,0,0-24,0v4H88V60h80v40ZM64,60.91V100H36V96A36.05,36.05,0,0,1,64,60.91ZM36,124H64v64H36Zm156,64V124h28v64Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M184,42H72A54.06,54.06,0,0,0,18,96v96a14,14,0,0,0,14,14H224a14,14,0,0,0,14-14V96A54.06,54.06,0,0,0,184,42Zm42,54v10H190V54.44A42.05,42.05,0,0,1,226,96Zm-88,42H118V102h20Zm-26,12h32a6,6,0,0,0,6-6V118h28v76H78V118h28v26A6,6,0,0,0,112,150Zm38-44V96a6,6,0,0,0-6-6H112a6,6,0,0,0-6,6v10H78V54H178v52ZM66,54.44V106H30V96A42.05,42.05,0,0,1,66,54.44ZM30,192V118H66v76H32A2,2,0,0,1,30,192Zm194,2H190V118h36v74A2,2,0,0,1,224,194Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M184,40H72A56.06,56.06,0,0,0,16,96v96a16,16,0,0,0,16,16H224a16,16,0,0,0,16-16V96A56.06,56.06,0,0,0,184,40Zm40,56v8H192V56.8A40.07,40.07,0,0,1,224,96Zm-88,40H120V104h16Zm-24,16h32a8,8,0,0,0,8-8V120h24v72H80V120h24v24A8,8,0,0,0,112,152Zm40-48V96a8,8,0,0,0-8-8H112a8,8,0,0,0-8,8v8H80V56h96v48ZM64,56.8V104H32V96A40.07,40.07,0,0,1,64,56.8ZM32,120H64v72H32Zm192,72H192V120h32v72Z"></path>
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
