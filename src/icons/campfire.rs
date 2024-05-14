//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[cfg(any(feature = "nature"))]
#[component]
pub fn Campfire(
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
                <path d="M132.19,25.19a8,8,0,0,0-8.38,0A156,156,0,0,0,96.24,48C77.77,67.13,68,87.9,68,108a60,60,0,0,0,120,0C188,60.08,134.47,26.59,132.19,25.19ZM128,152a24,24,0,0,1-24-24c0-24,24-40,24-40s24,16,24,40A24,24,0,0,1,128,152Zm95.62,74.42a8,8,0,0,1-10.05,5.2L128,204.39,42.43,231.62a8,8,0,1,1-4.85-15.25l64-20.37-64-20.38a8,8,0,1,1,4.85-15.24L128,187.6l85.57-27.22a8,8,0,1,1,4.85,15.24l-64,20.38,64,20.37A8,8,0,0,1,223.62,226.42Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path
        d="M180,108a52,52,0,0,1-52,52,24,24,0,0,0,24-24c0-24-24-40-24-40s-24,16-24,40a24,24,0,0,0,24,24,52,52,0,0,1-52-52c0-44,52-76,52-76S180,64,180,108Z"
        opacity="0.2"
    ></path>
    <path d="M128,168a60.07,60.07,0,0,0,60-60c0-47.92-53.53-81.41-55.81-82.81a8,8,0,0,0-8.38,0A156,156,0,0,0,96.24,48C77.77,67.13,68,87.9,68,108A60.07,60.07,0,0,0,128,168Zm-16-32c0-13.57,10-24.46,16-29.79,6,5.33,16,16.22,16,29.79a16,16,0,0,1-32,0Zm16-94.34C139.74,50,172,76,172,108a43.83,43.83,0,0,1-12.09,30.24c.05-.74.09-1.49.09-2.24,0-28-26.44-45.91-27.56-46.66a8,8,0,0,0-8.88,0C122.44,90.09,96,108,96,136c0,.75,0,1.5.09,2.24A43.83,43.83,0,0,1,84,108C84,76,116.27,50,128,41.66Zm95.62,184.76a8,8,0,0,1-10.05,5.2L128,204.39,42.43,231.62a8,8,0,1,1-4.85-15.25l64-20.37-64-20.38a8,8,0,1,1,4.85-15.24L128,187.6l85.57-27.22a8,8,0,1,1,4.85,15.24l-64,20.38,64,20.37A8,8,0,0,1,223.62,226.42Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M219.81,225.21A4,4,0,0,1,216,228a3.92,3.92,0,0,1-1.21-.19L128,200.2,41.21,227.81A3.92,3.92,0,0,1,40,228a4,4,0,0,1-1.21-7.81l76-24.19-76-24.19a4,4,0,1,1,2.42-7.62L128,191.8l86.79-27.61a4,4,0,1,1,2.42,7.62l-76,24.19,76,24.19A4,4,0,0,1,219.81,225.21ZM72,108c0-19,9.38-38.85,27.12-57.27A152,152,0,0,1,125.9,28.59a4,4,0,0,1,4.2,0,152,152,0,0,1,26.78,22.14C174.62,69.15,184,89,184,108a56,56,0,0,1-54.56,56c-.48,0-1,0-1.44,0s-1,0-1.44,0A56,56,0,0,1,72,108Zm56,48a20,20,0,0,0,20-20c0-17.39-14.37-30.53-20-35-5.63,4.48-20,17.62-20,35A20,20,0,0,0,128,156ZM80,108a48,48,0,0,0,23.28,41.13A27.83,27.83,0,0,1,100,136c0-25.84,24.73-42.63,25.78-43.33a4,4,0,0,1,4.44,0c1.05.7,25.78,17.49,25.78,43.33a27.83,27.83,0,0,1-3.28,13.13A48,48,0,0,0,176,108c0-36.37-38.49-64.76-48-71.21C118.5,43.25,80,71.68,80,108Z"></path>
}.into_view(),
IconWeight::Bold => view! {
    <path d="M227.51,227.4A12,12,0,0,1,216,236a12.15,12.15,0,0,1-3.4-.49l-84.6-25-84.6,25A12.15,12.15,0,0,1,40,236a12,12,0,0,1-3.4-23.51L85.65,198,36.6,183.51a12,12,0,0,1,6.8-23l84.6,25,84.6-25a12,12,0,1,1,6.8,23L170.35,198l49.05,14.49A12,12,0,0,1,227.51,227.4ZM64,104c0-50.59,55.93-81.28,58.31-82.57a12,12,0,0,1,11.38,0C136.07,22.72,192,53.41,192,104a64,64,0,0,1-128,0Zm64,40a12,12,0,0,0,12-12c0-6.47-2.71-12.55-8-18.07a41.26,41.26,0,0,0-4-3.57,42.49,42.49,0,0,0-4,3.57c-5.33,5.52-8,11.6-8,18.07A12,12,0,0,0,128,144ZM88,104a39.78,39.78,0,0,0,4.85,19.08c4.81-25,28.66-37.25,29.78-37.81a12,12,0,0,1,10.74,0c1.12.56,25,12.78,29.78,37.81A39.78,39.78,0,0,0,168,104c0-28.34-27.74-49.81-40-57.92C115.75,54.18,88,75.66,88,104Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M221.72,225.82a6,6,0,0,1-7.54,3.9L128,202.3,41.82,229.72a6,6,0,1,1-3.64-11.44l70-22.28-70-22.28a6,6,0,1,1,3.64-11.44L128,189.7l86.18-27.42a6,6,0,1,1,3.64,11.44l-70,22.28,70,22.28A6,6,0,0,1,221.72,225.82ZM70,108c0-46.81,52.62-79.73,54.86-81.11a6,6,0,0,1,6.28,0C133.38,28.27,186,61.19,186,108a58,58,0,0,1-116,0Zm58,46a18,18,0,0,0,18-18c0-15.48-12-27.43-18-32.44-6,5-18,17-18,32.44A18,18,0,0,0,128,154ZM82,108a45.93,45.93,0,0,0,17,35.67A29.87,29.87,0,0,1,98,136c0-26.9,25.58-44.27,26.67-45a6,6,0,0,1,6.66,0c1.09.72,26.67,18.09,26.67,45a29.87,29.87,0,0,1-1,7.67A45.93,45.93,0,0,0,174,108c0-34.06-35.15-61.22-46-68.78C117.15,46.78,82,73.93,82,108Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M223.62,226.42a8,8,0,0,1-10.05,5.2L128,204.39,42.43,231.62a8,8,0,1,1-4.85-15.25l64-20.37-64-20.38a8,8,0,1,1,4.85-15.24L128,187.6l85.57-27.22a8,8,0,1,1,4.85,15.24l-64,20.38,64,20.37A8,8,0,0,1,223.62,226.42ZM68,108c0-20.1,9.77-40.87,28.24-60a156,156,0,0,1,27.57-22.76,8,8,0,0,1,8.38,0C134.47,26.59,188,60.08,188,108a60,60,0,0,1-120,0Zm60,44a16,16,0,0,0,16-16c0-13.57-10-24.46-16-29.79-6,5.33-16,16.22-16,29.79A16,16,0,0,0,128,152ZM84,108a43.83,43.83,0,0,0,12.09,30.24c0-.74-.09-1.49-.09-2.24,0-28,26.44-45.91,27.56-46.66a8,8,0,0,1,8.88,0C133.56,90.09,160,108,160,136c0,.75,0,1.5-.09,2.24A43.83,43.83,0,0,0,172,108c0-32-32.26-58-44-66.34C116.27,50,84,76,84,108Z"></path>
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
