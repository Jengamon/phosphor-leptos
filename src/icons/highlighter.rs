//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[cfg(any(feature = "design", feature = "editor", feature = "office"))]
#[component]
pub fn Highlighter(
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
                <path d="M253.66,106.34a8,8,0,0,0-11.32,0L192,156.69,107.31,72l50.35-50.34a8,8,0,1,0-11.32-11.32L96,60.69A16,16,0,0,0,93.18,79.5L72,100.69a16,16,0,0,0,0,22.62L76.69,128,18.34,186.34a8,8,0,0,0,3.13,13.25l72,24A7.88,7.88,0,0,0,96,224a8,8,0,0,0,5.66-2.34L136,187.31l4.69,4.69a16,16,0,0,0,22.62,0l21.18-21.18A16,16,0,0,0,203.31,168l50.35-50.34A8,8,0,0,0,253.66,106.34ZM152,180.69,83.31,112,104,91.31,172.69,160Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path d="M136,176,96,216,24,192l64-64Z" opacity="0.2"></path>
    <path d="M253.66,106.34a8,8,0,0,0-11.32,0L192,156.69,109.66,74.34h0L107.31,72l50.35-50.34a8,8,0,1,0-11.32-11.32L96,60.69A16,16,0,0,0,93.18,79.5L72,100.69a16,16,0,0,0,0,22.62L76.69,128,18.34,186.34a8,8,0,0,0,3.13,13.25l72,24A7.88,7.88,0,0,0,96,224a8,8,0,0,0,5.66-2.34L136,187.31l4.69,4.69a16,16,0,0,0,22.62,0l21.18-21.18A16,16,0,0,0,203.31,168l50.35-50.34A8,8,0,0,0,253.66,106.34ZM93.84,206.85l-55-18.35L88,139.31,124.69,176ZM152,180.69l-10.34-10.35h0l-48-48h0L83.31,112,104,91.31,172.69,160Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M250.83,109.17a4,4,0,0,0-5.66,0l-50.34,50.34a4,4,0,0,1-5.66,0L104.49,74.83a4,4,0,0,1,0-5.66l50.34-50.34a4,4,0,0,0-5.66-5.66L98.83,63.51A12,12,0,0,0,98.37,80L74.83,103.51a12,12,0,0,0,0,17L82.34,128,21.17,189.17a4,4,0,0,0,1.57,6.62l72,24A3.92,3.92,0,0,0,96,220a4,4,0,0,0,2.83-1.17L136,181.66l7.51,7.51a12,12,0,0,0,17,0L184,165.63a12,12,0,0,0,16.47-.46l50.34-50.34A4,4,0,0,0,250.83,109.17ZM94.92,211.42,31.4,190.25,88,133.66,130.34,176Zm59.91-27.91a4,4,0,0,1-5.66,0L80.49,114.83a4,4,0,0,1,0-5.66L104,85.66,178.34,160Z"></path>
}.into_view(),
IconWeight::Bold => view! {
    <path d="M252.49,107.51a12,12,0,0,0-17,0L192,151,113,72l43.52-43.51a12,12,0,0,0-17-17L93.17,57.86a20,20,0,0,0-4.72,20.72L69.17,97.86a20,20,0,0,0,0,28.28L71,128,15.51,183.51a12,12,0,0,0,4.7,19.87l72,24A11.8,11.8,0,0,0,96,228a12,12,0,0,0,8.49-3.52L136,193l1.86,1.86a20,20,0,0,0,28.28,0l19.27-19.27a20.27,20.27,0,0,0,6.59,1.13,19.86,19.86,0,0,0,14.14-5.86l46.35-46.34A12,12,0,0,0,252.49,107.51ZM92.76,202.27,46.21,186.76,88,145l31,31ZM152,175,96.49,119.52h0L89,112l15-15,63,63Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M252.24,107.76a6,6,0,0,0-8.48,0L193.41,158.1a2,2,0,0,1-2.82,0L105.9,73.41a2,2,0,0,1,0-2.82l50.34-50.35a6,6,0,0,0-8.48-8.48L97.41,62.1A14,14,0,0,0,95.7,79.81L73.41,102.1a14,14,0,0,0,0,19.8l6.1,6.1L19.76,187.76a6,6,0,0,0,2.34,9.93l72,24a6,6,0,0,0,6.14-1.45L136,184.49l6.1,6.1a14,14,0,0,0,19.8,0l22.28-22.29a14,14,0,0,0,17.72-1.71l50.34-50.35A6,6,0,0,0,252.24,107.76ZM94.38,209.14,35.11,189.38,88,136.49,127.51,176Zm59-27a2,2,0,0,1-2.82,0l-10.35-10.34h0l-48-48h0L81.9,113.41a2,2,0,0,1,0-2.82L104,88.49,175.51,160Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M253.66,106.34a8,8,0,0,0-11.32,0L192,156.69,107.31,72l50.35-50.34a8,8,0,1,0-11.32-11.32L96,60.69A16,16,0,0,0,93.18,79.5L72,100.69a16,16,0,0,0,0,22.62L76.69,128,18.34,186.34a8,8,0,0,0,3.13,13.25l72,24A7.88,7.88,0,0,0,96,224a8,8,0,0,0,5.66-2.34L136,187.31l4.69,4.69a16,16,0,0,0,22.62,0l21.19-21.18A16,16,0,0,0,203.31,168l50.35-50.34A8,8,0,0,0,253.66,106.34ZM93.84,206.85l-55-18.35L88,139.31,124.69,176ZM152,180.69,83.31,112,104,91.31,172.69,160Z"></path>
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
