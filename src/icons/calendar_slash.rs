//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[cfg(any(feature = "office", feature = "system"))]
#[component]
pub fn CalendarSlash(
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
                <path d="M224,48V187.57a4,4,0,0,1-7,2.7L116.8,80H208V48H184v8a8,8,0,0,1-8.52,8A8.18,8.18,0,0,1,168,55.73V48H87.71l-8.46-9.31a4,4,0,0,1,3-6.69H168V24a8,8,0,0,1,8.52-8A8.18,8.18,0,0,1,184,24.27V32h24A16,16,0,0,1,224,48ZM213.92,210.62A8,8,0,0,1,208,224H48a16,16,0,0,1-16-16V48A16,16,0,0,1,48,32a8,8,0,0,1,5.93,2.62ZM73.55,80,48,51.89V80Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path d="M216,48V88H40V48a8,8,0,0,1,8-8H208A8,8,0,0,1,216,48Z" opacity="0.2"></path>
    <path d="M53.92,34.62A8,8,0,0,0,48,32,16,16,0,0,0,32,48V208a16,16,0,0,0,16,16H208a8,8,0,0,0,5.92-13.38ZM73.55,80H48V51.88ZM48,208V96H88.1L189.92,208ZM224,48V177.23a8,8,0,1,1-16,0V96H134.88a8,8,0,0,1,0-16H208V48H184v8a8,8,0,0,1-16,0V48H91.25a8,8,0,0,1,0-16H168V24a8,8,0,0,1,16,0v8h24A16,16,0,0,1,224,48Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M51,37.31A4,4,0,0,0,48,36,12,12,0,0,0,36,48V208a12,12,0,0,0,12,12H208a4,4,0,0,0,3-6.69Zm-4.46,7L82.59,84H44V48A4,4,0,0,1,46.5,44.29ZM48,212a4,4,0,0,1-4-4V92H89.87L199,212ZM220,48V177.23a4,4,0,1,1-8,0V92H134.88a4,4,0,0,1,0-8H212V48a4,4,0,0,0-4-4H180V56a4,4,0,0,1-8,0V44H91.25a4,4,0,0,1,0-8H172V24a4,4,0,0,1,8,0V36h28A12,12,0,0,1,220,48Z"></path>
}.into_view(),
IconWeight::Bold => view! {
    <path d="M56.88,31.93A12,12,0,0,0,48,28,20,20,0,0,0,28,48V208a20,20,0,0,0,20,20H208a12,12,0,0,0,8.88-20.07ZM52,204V62.24L180.87,204ZM228,48V165.34a12,12,0,1,1-24,0V100H145.69a12,12,0,0,1,0-24H204V52H188a12,12,0,0,1-24,0H102.06a12,12,0,0,1,0-24H164V24a12,12,0,0,1,24,0v4h20A20,20,0,0,1,228,48Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M52.44,36A6,6,0,0,0,48,34,14,14,0,0,0,34,48V208a14,14,0,0,0,14,14H208a6,6,0,0,0,4.44-10ZM46.26,47,78.07,82H46V48A2.06,2.06,0,0,1,46.26,47ZM48,210a2,2,0,0,1-2-2V94H89L194.44,210ZM222,48V177.23a6,6,0,1,1-12,0V94H134.88a6,6,0,0,1,0-12H210V48a2,2,0,0,0-2-2H182V56a6,6,0,0,1-12,0V46H91.25a6,6,0,0,1,0-12H170V24a6,6,0,0,1,12,0V34h26A14,14,0,0,1,222,48Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M53.92,34.62A8,8,0,0,0,48,32,16,16,0,0,0,32,48V208a16,16,0,0,0,16,16H208a8,8,0,0,0,5.92-13.38ZM73.55,80H48V51.88ZM48,208V96H88.1L189.92,208ZM224,48V177.23a8,8,0,1,1-16,0V96H134.88a8,8,0,0,1,0-16H208V48H184v8a8,8,0,0,1-16,0V48H91.25a8,8,0,0,1,0-16H168V24a8,8,0,0,1,16,0v8h24A16,16,0,0,1,224,48Z"></path>
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
