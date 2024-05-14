//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[cfg(any(feature = "arrows"))]
#[component]
pub fn ArrowsCounterClockwise(
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
                <path d="M88,104H40a8,8,0,0,1-8-8V48a8,8,0,0,1,13.66-5.66L64,60.7a95.42,95.42,0,0,1,66-26.76h.53a95.36,95.36,0,0,1,67.07,27.33,8,8,0,0,1-11.18,11.44,79.52,79.52,0,0,0-55.89-22.77h-.45A79.48,79.48,0,0,0,75.35,72L93.66,90.34A8,8,0,0,1,88,104Zm128,48H168a8,8,0,0,0-5.66,13.66L180.65,184a79.48,79.48,0,0,1-54.72,22.09h-.45a79.52,79.52,0,0,1-55.89-22.77,8,8,0,1,0-11.18,11.44,95.36,95.36,0,0,0,67.07,27.33H126a95.42,95.42,0,0,0,66-26.76l18.36,18.36A8,8,0,0,0,224,208V160A8,8,0,0,0,216,152Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path d="M216,128a88,88,0,1,1-88-88A88,88,0,0,1,216,128Z" opacity="0.2"></path>
    <path d="M88,104H40a8,8,0,0,1-8-8V48a8,8,0,0,1,16,0V76.69L62.63,62.06A95.43,95.43,0,0,1,130,33.94h.53a95.36,95.36,0,0,1,67.07,27.33,8,8,0,0,1-11.18,11.44,79.52,79.52,0,0,0-55.89-22.77h-.45A79.56,79.56,0,0,0,73.94,73.37L59.31,88H88a8,8,0,0,1,0,16Zm128,48H168a8,8,0,0,0,0,16h28.69l-14.63,14.63a79.56,79.56,0,0,1-56.13,23.43h-.45a79.52,79.52,0,0,1-55.89-22.77,8,8,0,1,0-11.18,11.44,95.36,95.36,0,0,0,67.07,27.33H126a95.43,95.43,0,0,0,67.36-28.12L208,179.31V208a8,8,0,0,0,16,0V160A8,8,0,0,0,216,152Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M88,100H40a4,4,0,0,1-4-4V48a4,4,0,0,1,8,0V86.34L65.46,64.89A91.39,91.39,0,0,1,130,37.94h.51A91.43,91.43,0,0,1,194.8,64.13a4,4,0,0,1-5.6,5.72,83.44,83.44,0,0,0-58.68-23.91h-.47a83.52,83.52,0,0,0-58.94,24.6L49.66,92H88a4,4,0,0,1,0,8Zm128,56H168a4,4,0,0,0,0,8h38.34l-21.45,21.46A83.52,83.52,0,0,1,126,210.06h-.47A83.44,83.44,0,0,1,66.8,186.15a4,4,0,0,0-5.6,5.72,91.43,91.43,0,0,0,64.28,26.19H126a91.39,91.39,0,0,0,64.55-26.95L212,169.66V208a4,4,0,0,0,8,0V160A4,4,0,0,0,216,156Z"></path>
}.into_view(),
IconWeight::Bold => view! {
    <path d="M88,108H40A12,12,0,0,1,28,96V48a12,12,0,0,1,24,0V67l7.8-7.8A99.42,99.42,0,0,1,130,29.94h.56a99.38,99.38,0,0,1,69.87,28.47,12,12,0,0,1-16.78,17.16,76,76,0,0,0-106.84.63L69,84H88a12,12,0,0,1,0,24Zm128,40H168a12,12,0,0,0,0,24h19l-7.8,7.8a75.55,75.55,0,0,1-53.32,22.26h-.43a75.49,75.49,0,0,1-53.09-21.63,12,12,0,0,0-16.78,17.16,99.38,99.38,0,0,0,69.87,28.47H126a99.42,99.42,0,0,0,70.16-29.29L204,189v19a12,12,0,0,0,24,0V160A12,12,0,0,0,216,148Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M88,102H40a6,6,0,0,1-6-6V48a6,6,0,0,1,12,0V81.52l18-18a93.45,93.45,0,0,1,66-27.53h.52A93.39,93.39,0,0,1,196.19,62.7a6,6,0,0,1-8.38,8.58A82,82,0,0,0,72.53,72L54.48,90H88a6,6,0,0,1,0,12Zm128,52H168a6,6,0,0,0,0,12h33.52l-18.05,18a81.51,81.51,0,0,1-57.53,24h-.46a81.5,81.5,0,0,1-57.29-23.34,6,6,0,0,0-8.38,8.58,93.39,93.39,0,0,0,65.67,26.76H126a93.45,93.45,0,0,0,66-27.53l18-18.05V208a6,6,0,0,0,12,0V160A6,6,0,0,0,216,154Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M88,104H40a8,8,0,0,1-8-8V48a8,8,0,0,1,16,0V76.69L62.63,62.06A95.43,95.43,0,0,1,130,33.94h.53a95.36,95.36,0,0,1,67.07,27.33,8,8,0,0,1-11.18,11.44,79.52,79.52,0,0,0-55.89-22.77h-.45A79.56,79.56,0,0,0,73.94,73.37L59.31,88H88a8,8,0,0,1,0,16Zm128,48H168a8,8,0,0,0,0,16h28.69l-14.63,14.63a79.56,79.56,0,0,1-56.13,23.43h-.45a79.52,79.52,0,0,1-55.89-22.77,8,8,0,1,0-11.18,11.44,95.36,95.36,0,0,0,67.07,27.33H126a95.43,95.43,0,0,0,67.36-28.12L208,179.31V208a8,8,0,0,0,16,0V160A8,8,0,0,0,216,152Z"></path>
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
