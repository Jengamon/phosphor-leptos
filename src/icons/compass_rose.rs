//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[cfg(any(feature = "map", feature = "objects"))]
#[component]
pub fn CompassRose(
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
                <path d="M249.94,120.24l-27.05-6.76a95.86,95.86,0,0,0-80.37-80.37l-6.76-27a8,8,0,0,0-15.52,0l-6.76,27.05a95.86,95.86,0,0,0-80.37,80.37l-27,6.76a8,8,0,0,0,0,15.52l27.05,6.76a95.86,95.86,0,0,0,80.37,80.37l6.76,27.05a8,8,0,0,0,15.52,0l6.76-27.05a95.86,95.86,0,0,0,80.37-80.37l27.05-6.76a8,8,0,0,0,0-15.52Zm-44.17-11L158.6,97.4,146.8,50.23A79.88,79.88,0,0,1,205.77,109.2Zm-96.57-59L97.4,97.4,50.23,109.2A79.88,79.88,0,0,1,109.2,50.23Zm-59,96.57L97.4,158.6l11.8,47.17A79.88,79.88,0,0,1,50.23,146.8Zm96.57,59,11.8-47.17,47.17-11.8A79.88,79.88,0,0,1,146.8,205.77Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path d="M248,128l-96,24-24,96-24-96L8,128l96-24L128,8l24,96Z" opacity="0.2"></path>
    <path d="M249.94,120.24l-27.05-6.76a95.86,95.86,0,0,0-80.37-80.37l-6.76-27a8,8,0,0,0-15.52,0l-6.76,27.05a95.86,95.86,0,0,0-80.37,80.37l-27,6.76a8,8,0,0,0,0,15.52l27.05,6.76a95.86,95.86,0,0,0,80.37,80.37l6.76,27.05a8,8,0,0,0,15.52,0l6.76-27.05a95.86,95.86,0,0,0,80.37-80.37l27.05-6.76a8,8,0,0,0,0-15.52Zm-95.49,22.9L139.31,128l15.14-15.14L215,128Zm-52.9,0L41,128l60.57-15.14L116.69,128ZM205.77,109.2,158.6,97.4,146.8,50.23A79.88,79.88,0,0,1,205.77,109.2Zm-62.63-7.65L128,116.69l-15.14-15.14L128,41ZM109.2,50.23,97.4,97.4,50.23,109.2A79.88,79.88,0,0,1,109.2,50.23Zm-59,96.57L97.4,158.6l11.8,47.17A79.88,79.88,0,0,1,50.23,146.8Zm62.63,7.65L128,139.31l15.14,15.14L128,215Zm33.94,51.32,11.8-47.17,47.17-11.8A79.88,79.88,0,0,1,146.8,205.77Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M249,124.12l-29.68-7.42a91.84,91.84,0,0,0-80-80L131.88,7a4,4,0,0,0-7.76,0L116.7,36.71a91.84,91.84,0,0,0-80,80L7,124.12a4,4,0,0,0,0,7.76l29.68,7.42a91.84,91.84,0,0,0,80,80L124.12,249a4,4,0,0,0,7.76,0l7.42-29.68a91.84,91.84,0,0,0,80-80L249,131.88a4,4,0,0,0,0-7.76Zm-38.05-9.51L155.3,100.7,141.39,45.08A83.85,83.85,0,0,1,210.92,114.61ZM128,24.49l19.57,78.28L128,122.34l-19.57-19.57ZM114.61,45.08,100.7,100.7,45.08,114.61A83.85,83.85,0,0,1,114.61,45.08Zm-11.84,63.35L122.34,128l-19.57,19.57L24.49,128Zm-57.69,33L100.7,155.3l13.91,55.62A83.85,83.85,0,0,1,45.08,141.39ZM128,231.51l-19.57-78.28L128,133.66l19.57,19.57Zm13.39-20.59L155.3,155.3l55.62-13.91A83.85,83.85,0,0,1,141.39,210.92Zm11.84-63.35L133.66,128l19.57-19.57L231.51,128Z"></path>
}.into_view(),
IconWeight::Bold => view! {
    <path d="M247,116.39l-20.47-5.34A100.27,100.27,0,0,0,145,29.44L139.61,9a12,12,0,0,0-23.22,0l-5.34,20.47a100.27,100.27,0,0,0-81.61,81.61L9,116.39a12,12,0,0,0,0,23.22L29.44,145a100.27,100.27,0,0,0,81.61,81.61L116.39,247a12,12,0,0,0,23.22,0L145,226.56A100.27,100.27,0,0,0,226.56,145L247,139.61a12,12,0,0,0,0-23.22Zm-46.88-12.23-38.31-10-10-38.31A76.32,76.32,0,0,1,200.15,104.16Zm-82.8-3.78L128,59.54l10.65,40.84L128,111ZM128,145l10.65,10.65L128,196.46l-10.65-40.84Zm-27.62-27.62L111,128l-10.65,10.65L59.54,128Zm55.24,21.3L145,128l10.65-10.65L196.46,128Zm-51.46-82.8-10,38.31-38.31,10A76.32,76.32,0,0,1,104.16,55.85Zm-48.31,96,38.31,10,10,38.31A76.32,76.32,0,0,1,55.85,151.84Zm96,48.31,10-38.31,38.31-10A76.32,76.32,0,0,1,151.84,200.15Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M249.46,122.18l-28.34-7.09A93.87,93.87,0,0,0,140.9,34.88L133.82,6.54a6,6,0,0,0-11.64,0L115.1,34.88a93.87,93.87,0,0,0-80.22,80.21L6.54,122.18a6,6,0,0,0,0,11.64l28.34,7.09a93.87,93.87,0,0,0,80.22,80.21l7.08,28.34a6,6,0,0,0,11.64,0l7.08-28.34a93.87,93.87,0,0,0,80.22-80.21l28.34-7.09a6,6,0,0,0,0-11.64Zm-41.05-10.26L157,99.05,144.08,47.59A81.87,81.87,0,0,1,208.41,111.92Zm-63-9.76L128,119.51l-17.36-17.35L128,32.74ZM111.92,47.59,99.05,99.05,47.59,111.92A81.87,81.87,0,0,1,111.92,47.59Zm-9.76,63.06L119.52,128l-17.36,17.35L32.74,128ZM47.59,144.08,99.05,157l12.87,51.46A81.87,81.87,0,0,1,47.59,144.08Zm63.05,9.76L128,136.49l17.36,17.35L128,223.26Zm33.44,54.57L157,157l51.46-12.87A81.87,81.87,0,0,1,144.08,208.41Zm9.76-63.06L136.48,128l17.36-17.35L223.26,128Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M249.94,120.24l-27.05-6.76a95.86,95.86,0,0,0-80.37-80.37l-6.76-27a8,8,0,0,0-15.52,0l-6.76,27.05a95.86,95.86,0,0,0-80.37,80.37l-27,6.76a8,8,0,0,0,0,15.52l27.05,6.76a95.86,95.86,0,0,0,80.37,80.37l6.76,27.05a8,8,0,0,0,15.52,0l6.76-27.05a95.86,95.86,0,0,0,80.37-80.37l27.05-6.76a8,8,0,0,0,0-15.52Zm-95.49,22.9L139.31,128l15.14-15.14L215,128Zm-52.9,0L41,128l60.57-15.14L116.69,128ZM205.77,109.2,158.6,97.4,146.8,50.23A79.88,79.88,0,0,1,205.77,109.2Zm-62.63-7.65L128,116.69l-15.14-15.14L128,41ZM109.2,50.23,97.4,97.4,50.23,109.2A79.88,79.88,0,0,1,109.2,50.23Zm-59,96.57L97.4,158.6l11.8,47.17A79.88,79.88,0,0,1,50.23,146.8Zm62.63,7.65L128,139.31l15.14,15.14L128,215Zm33.94,51.32,11.8-47.17,47.17-11.8A79.88,79.88,0,0,1,146.8,205.77Z"></path>
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
