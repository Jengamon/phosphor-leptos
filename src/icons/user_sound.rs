//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[cfg(any(feature = "people"))]
#[component]
pub fn UserSound(
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
                <path d="M198.13,202.85A8,8,0,0,1,192,216H24a8,8,0,0,1-6.12-13.15c14.94-17.78,33.52-30.41,54.17-37.17a68,68,0,1,1,71.9,0C164.6,172.44,183.18,185.07,198.13,202.85ZM196.86,61.39a8,8,0,0,0-4.22,10.5,92.26,92.26,0,0,1,0,72.22,8,8,0,1,0,14.72,6.29,108.36,108.36,0,0,0,0-84.8A8,8,0,0,0,196.86,61.39Zm39.85-8.54a8,8,0,1,0-14.7,6.3,124.43,124.43,0,0,1,0,97.7,8,8,0,1,0,14.7,6.3,140.34,140.34,0,0,0,0-110.3Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path d="M168,108a60,60,0,1,1-60-60A60,60,0,0,1,168,108Z" opacity="0.2"></path>
    <path d="M144,165.68a68,68,0,1,0-71.9,0c-20.65,6.76-39.23,19.39-54.17,37.17a8,8,0,0,0,12.25,10.3C50.25,189.19,77.91,176,108,176s57.75,13.19,77.88,37.15a8,8,0,1,0,12.25-10.3C183.18,185.07,164.6,172.44,144,165.68ZM56,108a52,52,0,1,1,52,52A52.06,52.06,0,0,1,56,108ZM207.36,65.6a108.36,108.36,0,0,1,0,84.8,8,8,0,0,1-7.36,4.86,8,8,0,0,1-7.36-11.15,92.26,92.26,0,0,0,0-72.22,8,8,0,0,1,14.72-6.29ZM248,108a139,139,0,0,1-11.29,55.15,8,8,0,0,1-14.7-6.3,124.43,124.43,0,0,0,0-97.7,8,8,0,1,1,14.7-6.3A139,139,0,0,1,248,108Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M133.17,166.84a64,64,0,1,0-50.34,0c-23.76,5.46-45.18,18.69-61.89,38.59a4,4,0,1,0,6.12,5.14C48,185.7,76.71,172,108,172s60,13.7,80.94,38.57a4,4,0,0,0,6.12-5.14C178.35,185.53,156.93,172.3,133.17,166.84ZM52,108a56,56,0,1,1,56,56A56.06,56.06,0,0,1,52,108ZM203.68,67.17a104.35,104.35,0,0,1,0,81.66,4,4,0,0,1-3.68,2.43,4.12,4.12,0,0,1-1.57-.32,4,4,0,0,1-2.11-5.25,96.32,96.32,0,0,0,0-75.38,4,4,0,0,1,7.36-3.14ZM244,108a135.2,135.2,0,0,1-11,53.58,4,4,0,0,1-3.68,2.42,3.94,3.94,0,0,1-1.57-.32,4,4,0,0,1-2.1-5.26,128.44,128.44,0,0,0,0-100.84A4,4,0,1,1,233,54.42,135.2,135.2,0,0,1,244,108Z"></path>
}.into_view(),
IconWeight::Bold => view! {
    <path d="M152.5,164.53a72,72,0,1,0-89,0,124.08,124.08,0,0,0-48.69,35.75,12,12,0,0,0,18.38,15.44C46.88,199.42,71,180,108,180s61.12,19.42,74.81,35.72a12,12,0,1,0,18.38-15.44A124,124,0,0,0,152.5,164.53ZM60,108a48,48,0,1,1,48,48A48.05,48.05,0,0,1,60,108Zm192,0a143.09,143.09,0,0,1-11.61,56.73,12,12,0,1,1-22-9.46,120.48,120.48,0,0,0,0-94.54,12,12,0,1,1,22-9.46A143.09,143.09,0,0,1,252,108ZM207,64.76a108.26,108.26,0,0,1,0,86.48,12,12,0,0,1-22-9.62,84.35,84.35,0,0,0,0-67.24,12,12,0,1,1,22-9.62Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M139,166.26a66,66,0,1,0-62,0c-22,6.22-41.88,19.15-57.61,37.88a6,6,0,0,0,9.18,7.72C49.11,187.45,77.31,174,108,174s58.9,13.45,79.41,37.86a6,6,0,1,0,9.18-7.72C180.86,185.41,161,172.48,139,166.26ZM54,108a54,54,0,1,1,54,54A54.06,54.06,0,0,1,54,108ZM205.52,66.39a106.33,106.33,0,0,1,0,83.22,6,6,0,0,1-11-4.71,94.29,94.29,0,0,0,0-73.8,6,6,0,0,1,11-4.71ZM246,108a137.16,137.16,0,0,1-11.12,54.37,6,6,0,0,1-11-4.74,126.41,126.41,0,0,0,0-99.26,6,6,0,0,1,11-4.74A137.16,137.16,0,0,1,246,108Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M144,165.68a68,68,0,1,0-71.9,0c-20.65,6.76-39.23,19.39-54.17,37.17a8,8,0,0,0,12.25,10.3C50.25,189.19,77.91,176,108,176s57.75,13.19,77.88,37.15a8,8,0,1,0,12.25-10.3C183.18,185.07,164.6,172.44,144,165.68ZM56,108a52,52,0,1,1,52,52A52.06,52.06,0,0,1,56,108ZM207.36,65.6a108.36,108.36,0,0,1,0,84.8,8,8,0,0,1-7.36,4.86,8,8,0,0,1-7.36-11.15,92.26,92.26,0,0,0,0-72.22,8,8,0,0,1,14.72-6.29ZM248,108a139,139,0,0,1-11.29,55.15,8,8,0,0,1-14.7-6.3,124.43,124.43,0,0,0,0-97.7,8,8,0,1,1,14.7-6.3A139,139,0,0,1,248,108Z"></path>
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
