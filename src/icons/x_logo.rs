//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[cfg(any(feature = "brand", feature = "communication"))]
#[component]
pub fn XLogo(
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
                <path d="M215,219.85a8,8,0,0,1-7,4.15H160a8,8,0,0,1-6.75-3.71l-40.49-63.63L53.92,221.38a8,8,0,0,1-11.84-10.76l61.77-68L41.25,44.3A8,8,0,0,1,48,32H96a8,8,0,0,1,6.75,3.71l40.49,63.63,58.84-64.72a8,8,0,0,1,11.84,10.76l-61.77,67.95,62.6,98.38A8,8,0,0,1,215,219.85Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path d="M208,216H160L48,40H96Z" opacity="0.2"></path>
    <path d="M214.75,211.71l-62.6-98.38,61.77-67.95a8,8,0,0,0-11.84-10.76L143.24,99.34,102.75,35.71A8,8,0,0,0,96,32H48a8,8,0,0,0-6.75,12.3l62.6,98.37-61.77,68a8,8,0,1,0,11.84,10.76l58.84-64.72,40.49,63.63A8,8,0,0,0,160,224h48a8,8,0,0,0,6.75-12.29ZM164.39,208,62.57,48h29L193.43,208Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M211.37,213.85,147.13,112.9,211,42.69A4,4,0,0,0,205,37.31L142.68,105.9,99.38,37.85A4,4,0,0,0,96,36H48a4,4,0,0,0-3.37,6.15L108.87,143.1,45,213.31A4,4,0,1,0,51,218.69l62.36-68.59,43.3,68.05A4,4,0,0,0,160,220h48a4,4,0,0,0,3.37-6.15ZM162.2,212,55.29,44H93.8L200.71,212Z"></path>
}.into_view(),
IconWeight::Bold => view! {
    <path d="M218.12,209.56l-61-95.8,59.72-65.69a12,12,0,0,0-17.76-16.14L143.81,92.77,106.12,33.56A12,12,0,0,0,96,28H48A12,12,0,0,0,37.88,46.44l61,95.8L39.12,207.93a12,12,0,1,0,17.76,16.14l55.31-60.84,37.69,59.21A12,12,0,0,0,160,228h48a12,12,0,0,0,10.12-18.44ZM166.59,204,69.86,52H89.41l96.73,152Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M213.06,212.78l-63.42-99.66L212.44,44A6,6,0,1,0,203.56,36L143,102.62l-41.9-65.84A6,6,0,0,0,96,34H48a6,6,0,0,0-5.06,9.22l63.42,99.66L43.56,212A6,6,0,0,0,52.44,220L113,153.38l41.9,65.84A6,6,0,0,0,160,222h48a6,6,0,0,0,5.06-9.22ZM163.29,210,58.93,46H92.71L197.07,210Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M214.75,211.71l-62.6-98.38,61.77-67.95a8,8,0,0,0-11.84-10.76L143.24,99.34,102.75,35.71A8,8,0,0,0,96,32H48a8,8,0,0,0-6.75,12.3l62.6,98.37-61.77,68a8,8,0,1,0,11.84,10.76l58.84-64.72,40.49,63.63A8,8,0,0,0,160,224h48a8,8,0,0,0,6.75-12.29ZM164.39,208,62.57,48h29L193.43,208Z"></path>
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
            id=move || id.get().map(|id| id.get())
            class=move || class.get().map(|cls| cls.get())
        >
            {body}
        </svg>
    }
}
