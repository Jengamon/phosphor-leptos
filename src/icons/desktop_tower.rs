//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[cfg(any(feature = "development", feature = "objects"))]
#[component]
pub fn DesktopTower(
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
                <path d="M120,76V188a4,4,0,0,1-4,4H96v16h15.73a8.18,8.18,0,0,1,8.25,7.47,8,8,0,0,1-8,8.53H64.27A8.18,8.18,0,0,1,56,216.53,8,8,0,0,1,64,208H80V192H32A24,24,0,0,1,8,168V96A24,24,0,0,1,32,72h84A4,4,0,0,1,120,76ZM248,48V208a16,16,0,0,1-16,16H152a16,16,0,0,1-16-16V48a16,16,0,0,1,16-16h80A16,16,0,0,1,248,48ZM203.9,181.57a12,12,0,1,0-10.34,10.33A12,12,0,0,0,203.9,181.57ZM224,103.47A8.18,8.18,0,0,0,215.73,96H168.27a8.18,8.18,0,0,0-8.25,7.47,8,8,0,0,0,8,8.53h48A8,8,0,0,0,224,103.47Zm0-32A8.18,8.18,0,0,0,215.73,64H168.27A8.18,8.18,0,0,0,160,71.47,8,8,0,0,0,168,80h48A8,8,0,0,0,224,71.47Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path
        d="M240,48V208a8,8,0,0,1-8,8H152a8,8,0,0,1-8-8V48a8,8,0,0,1,8-8h80A8,8,0,0,1,240,48Z"
        opacity="0.2"
    ></path>
    <path d="M216,72a8,8,0,0,1-8,8H176a8,8,0,0,1,0-16h32A8,8,0,0,1,216,72Zm-8,24H176a8,8,0,0,0,0,16h32a8,8,0,0,0,0-16Zm40-48V208a16,16,0,0,1-16,16H152a16,16,0,0,1-16-16V192H96v16h16a8,8,0,0,1,0,16H64a8,8,0,0,1,0-16H80V192H32A24,24,0,0,1,8,168V96A24,24,0,0,1,32,72H136V48a16,16,0,0,1,16-16h80A16,16,0,0,1,248,48ZM136,176V88H32a8,8,0,0,0-8,8v72a8,8,0,0,0,8,8Zm96,32V48H152V208h80Zm-40-40a12,12,0,1,0,12,12A12,12,0,0,0,192,168Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M212,72a4,4,0,0,1-4,4H176a4,4,0,0,1,0-8h32A4,4,0,0,1,212,72Zm-4,28H176a4,4,0,0,0,0,8h32a4,4,0,0,0,0-8Zm36-52V208a12,12,0,0,1-12,12H152a12,12,0,0,1-12-12V188H92v24h20a4,4,0,0,1,0,8H64a4,4,0,0,1,0-8H84V188H32a20,20,0,0,1-20-20V96A20,20,0,0,1,32,76H140V48a12,12,0,0,1,12-12h80A12,12,0,0,1,244,48ZM140,180V84H32A12,12,0,0,0,20,96v72a12,12,0,0,0,12,12ZM236,48a4,4,0,0,0-4-4H152a4,4,0,0,0-4,4V208a4,4,0,0,0,4,4h80a4,4,0,0,0,4-4ZM192,172a8,8,0,1,0,8,8A8,8,0,0,0,192,172Z"></path>
}.into_view(),
IconWeight::Bold => view! {
    <path d="M168,76a12,12,0,0,1,12-12h24a12,12,0,0,1,0,24H180A12,12,0,0,1,168,76Zm12,48h24a12,12,0,0,0,0-24H180a12,12,0,0,0,0,24Zm72-76V208a20,20,0,0,1-20,20H152a20,20,0,0,1-20-20V192H100v12h8a12,12,0,0,1,0,24H68a12,12,0,0,1,0-24h8V192H32A28,28,0,0,1,4,164V96A28,28,0,0,1,32,68H132V48a20,20,0,0,1,20-20h80A20,20,0,0,1,252,48ZM132,168V92H32a4,4,0,0,0-4,4v68a4,4,0,0,0,4,4ZM228,52H156V204h72ZM192,160a16,16,0,1,0,16,16A16,16,0,0,0,192,160Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M214,72a6,6,0,0,1-6,6H176a6,6,0,0,1,0-12h32A6,6,0,0,1,214,72Zm-6,26H176a6,6,0,0,0,0,12h32a6,6,0,0,0,0-12Zm38-50V208a14,14,0,0,1-14,14H152a14,14,0,0,1-14-14V190H94v20h18a6,6,0,0,1,0,12H64a6,6,0,0,1,0-12H82V190H32a22,22,0,0,1-22-22V96A22,22,0,0,1,32,74H138V48a14,14,0,0,1,14-14h80A14,14,0,0,1,246,48ZM138,178V86H32A10,10,0,0,0,22,96v72a10,10,0,0,0,10,10ZM234,48a2,2,0,0,0-2-2H152a2,2,0,0,0-2,2V208a2,2,0,0,0,2,2h80a2,2,0,0,0,2-2ZM192,170a10,10,0,1,0,10,10A10,10,0,0,0,192,170Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M216,72a8,8,0,0,1-8,8H176a8,8,0,0,1,0-16h32A8,8,0,0,1,216,72Zm-8,24H176a8,8,0,0,0,0,16h32a8,8,0,0,0,0-16Zm40-48V208a16,16,0,0,1-16,16H152a16,16,0,0,1-16-16V192H96v16h16a8,8,0,0,1,0,16H64a8,8,0,0,1,0-16H80V192H32A24,24,0,0,1,8,168V96A24,24,0,0,1,32,72H136V48a16,16,0,0,1,16-16h80A16,16,0,0,1,248,48ZM136,176V88H32a8,8,0,0,0-8,8v72a8,8,0,0,0,8,8Zm96,32V48H152V208h80Zm-40-40a12,12,0,1,0,12,12A12,12,0,0,0,192,168Z"></path>
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
