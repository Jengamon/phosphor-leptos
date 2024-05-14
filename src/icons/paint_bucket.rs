//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[cfg(any(feature = "design", feature = "editor", feature = "objects"))]
#[component]
pub fn PaintBucket(
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
                <path d="M256,208a24,24,0,0,1-48,0c0-17.91,15.57-41.77,17.34-44.44a8,8,0,0,1,13.32,0C240.43,166.23,256,190.09,256,208ZM132.49,124.49a12,12,0,0,0-17-17l0,0s0,0,0,0a12,12,0,0,0,17,16.94ZM37.65,18.34A8,8,0,0,0,26.34,29.66l32.6,32.6L70.25,51ZM234.53,139.07a8,8,0,0,0,3.13-13.24L122.17,10.34a8,8,0,0,0-11.31,0L70.25,51l40.43,40.42a28,28,0,1,1-11.31,11.32L58.94,62.26,15,106.17a24,24,0,0,0,0,33.94L99.89,225a24,24,0,0,0,33.94,0l78.49-78.49Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path
        d="M248,208a16,16,0,0,1-32,0c0-16,16-40,16-40S248,192,248,208Zm-16-76.52-24,8-79.83,79.83a16,16,0,0,1-22.63,0L20.69,134.46a16,16,0,0,1,0-22.63L116.52,16Zm-93.86-29.62a20,20,0,1,0,0,28.28A20,20,0,0,0,138.14,101.86Z"
        opacity="0.2"
    ></path>
    <path d="M238.66,163.56a8,8,0,0,0-13.32,0C223.57,166.23,208,190.09,208,208a24,24,0,0,0,48,0C256,190.09,240.43,166.23,238.66,163.56ZM232,216a8,8,0,0,1-8-8c0-6.8,4-16.32,8-24.08,4,7.76,8,17.34,8,24.08A8,8,0,0,1,232,216Zm2.53-76.93a8,8,0,0,0,3.13-13.24L122.17,10.34a8,8,0,0,0-11.31,0L70.25,51,45.65,26.34A8,8,0,0,0,34.34,37.66l24.6,24.6L15,106.17a24,24,0,0,0,0,33.94L99.89,225a24,24,0,0,0,33.94,0l78.49-78.49Zm-32.19-5.24-79.83,79.83a8,8,0,0,1-11.31,0L26.34,128.8a8,8,0,0,1,0-11.31L70.25,73.57l29.12,29.12a28,28,0,1,0,11.31-11.32L81.57,62.26l35-34.95L217.19,128l-11.72,3.9A8.09,8.09,0,0,0,202.34,133.83Zm-86.83-26.31,0,0a13.26,13.26,0,1,1-.05.06S115.51,107.53,115.51,107.52Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M235.33,165.78a4,4,0,0,0-6.66,0C228,166.8,212,191,212,208a20,20,0,0,0,40,0C252,191,236,166.8,235.33,165.78ZM232,220a12,12,0,0,1-12-12c0-10.25,7.49-24.64,12-32.37,4.51,7.73,12,22.1,12,32.37A12,12,0,0,1,232,220Zm3.89-87.6a4,4,0,0,0-1.06-3.74L119.34,13.17a4,4,0,0,0-5.65,0L70.25,56.6,42.82,29.17a4,4,0,0,0-5.65,5.66L64.6,62.26,17.86,109a20,20,0,0,0,0,28.29l84.85,84.85a20,20,0,0,0,28.29,0L210.16,143l23.1-7.7A4,4,0,0,0,235.89,132.4Zm-29.15,3.29a4.06,4.06,0,0,0-1.57,1l-79.83,79.82a12,12,0,0,1-17,0L23.51,131.63a12,12,0,0,1,0-17L70.25,67.92l34.2,34.2A24,24,0,0,0,141,133h0a24,24,0,0,0-30.86-36.51l-34.2-34.2,40.61-40.6L224.59,129.74Zm-94.05-31a0,0,0,0,0,0,0,16,16,0,1,1,0,22.64,16,16,0,0,1,0-22.64Z"></path>
}.into_view(),
IconWeight::Bold => view! {
    <path d="M235.79,142.88a12,12,0,0,0,4.7-19.87L125,7.52a12,12,0,0,0-17,0L70.25,45.29,48.48,23.52a12,12,0,0,0-17,17L53.28,62.26,12.2,103.35a28,28,0,0,0,0,39.6l84.86,84.86a28,28,0,0,0,39.6,0L214.48,150Zm-31.58-14.77a12,12,0,0,0-4.7,2.9l-79.82,79.83a4,4,0,0,1-5.66,0L29.17,126a4,4,0,0,1,0-5.66L70.25,79.24l24.29,24.29a32,32,0,0,0,52.09,35.11h0a32,32,0,0,0-35.12-52.08L87.23,62.26,116.52,33l93.27,93.28Zm-85.87-17.75,0,0a8,8,0,1,1-.06.06ZM256,208a24,24,0,0,1-48,0c0-19.44,12.93-37.23,14.4-39.2a12,12,0,0,1,19.2,0C243.07,170.78,256,188.57,256,208Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M237,164.67a6,6,0,0,0-10,0c-.7,1-17,25.72-17,43.33a22,22,0,0,0,44,0C254,190.39,237.69,165.71,237,164.67ZM232,218a10,10,0,0,1-10-10c0-8.17,5.37-19.92,10-28.34,4.63,8.41,10,20.15,10,28.34A10,10,0,0,1,232,218Zm1.9-80.82a6,6,0,0,0,2.34-9.94L120.76,11.76a6,6,0,0,0-8.49,0l-42,42-26-26a6,6,0,0,0-8.49,8.48l26,26L16.44,107.59a22,22,0,0,0,0,31.11l84.86,84.86a22,22,0,0,0,31.11,0l78.83-78.83Zm-30.14-1.94-79.83,79.83a10,10,0,0,1-14.14,0L24.93,130.21a10,10,0,0,1,0-14.14L70.25,70.75l31.62,31.61a26,26,0,0,0,3.75,32,26,26,0,0,0,36.76,0h0a26,26,0,0,0-32-40.51L78.74,62.26l37.78-37.77L220.89,128.86l-14.79,4.93A6.07,6.07,0,0,0,203.76,135.24ZM114.1,106.11l0,0a14,14,0,1,1,0,19.82,13.91,13.91,0,0,1,0-19.82Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M234.53,139.07a8,8,0,0,0,3.13-13.24L122.17,10.34a8,8,0,0,0-11.31,0L70.25,51,45.65,26.34A8,8,0,0,0,34.34,37.66l24.6,24.6L15,106.17a24,24,0,0,0,0,33.94L99.89,225a24,24,0,0,0,33.94,0l78.49-78.49Zm-32.19-5.24-79.83,79.83a8,8,0,0,1-11.31,0L26.34,128.8a8,8,0,0,1,0-11.31L70.25,73.57l29.12,29.12a28,28,0,1,0,11.31-11.32L81.57,62.26l35-34.95L217.19,128l-11.72,3.9A8.09,8.09,0,0,0,202.34,133.83Zm-86.83-26.31,0,0a13.26,13.26,0,1,1-.05.06S115.51,107.53,115.51,107.52Zm123.15,56a8,8,0,0,0-13.32,0C223.57,166.23,208,190.09,208,208a24,24,0,0,0,48,0C256,190.09,240.43,166.23,238.66,163.56ZM232,216a8,8,0,0,1-8-8c0-6.8,4-16.32,8-24.08,4,7.76,8,17.34,8,24.08A8,8,0,0,1,232,216Z"></path>
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
