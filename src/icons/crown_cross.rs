//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[cfg(any(feature = "games", feature = "objects"))]
#[component]
pub fn CrownCross(
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
                <path d="M128,83.22a53.86,53.86,0,0,0-8-10.06V40H104a8,8,0,0,1,0-16h16V8a8,8,0,0,1,16,0V24h16a8,8,0,0,1,0,16H136V73.16A53.86,53.86,0,0,0,128,83.22ZM180,56c-17.74,0-33.21,6.48-44,17.16V176a8,8,0,0,1-16,0V73.16C109.21,62.48,93.74,56,76,56a60.07,60.07,0,0,0-60,60c0,29.86,14.54,48.85,26.73,59.52A90.48,90.48,0,0,0,64,189.34V208a16,16,0,0,0,16,16h96a16,16,0,0,0,16-16V189.34a90.48,90.48,0,0,0,21.27-13.82C225.46,164.85,240,145.86,240,116A60.07,60.07,0,0,0,180,56Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path
        d="M232,116c0,52-48,68-48,68v24a8,8,0,0,1-8,8H80a8,8,0,0,1-8-8V184s-48-16-48-68A52,52,0,0,1,76,64c28.72,0,52,19.28,52,48,0-28.72,23.28-48,52-48A52,52,0,0,1,232,116Z"
        opacity="0.2"
    ></path>
    <path d="M180,56c-17.74,0-33.21,6.48-44,17.16V40h16a8,8,0,0,0,0-16H136V8a8,8,0,0,0-16,0V24H104a8,8,0,0,0,0,16h16V73.16C109.21,62.48,93.74,56,76,56a60.07,60.07,0,0,0-60,60c0,29.86,14.54,48.85,26.73,59.52A90.48,90.48,0,0,0,64,189.34V208a16,16,0,0,0,16,16h96a16,16,0,0,0,16-16V189.34a90.48,90.48,0,0,0,21.27-13.82C225.46,164.85,240,145.86,240,116A60.07,60.07,0,0,0,180,56Zm1.47,120.41A8,8,0,0,0,176,184v24H80V184a8,8,0,0,0-5.47-7.59C74.1,176.27,32,161.7,32,116A44.05,44.05,0,0,1,76,72c25.5,0,44,16.82,44,40v64a8,8,0,0,0,16,0V112c0-23.18,18.5-40,44-40a44.05,44.05,0,0,1,44,44C224,161.4,183.18,175.83,181.47,176.41Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M180,60c-15.15,0-29.15,5.06-39.43,14.25a49.31,49.31,0,0,0-8.57,10V36h20a4,4,0,0,0,0-8H132V8a4,4,0,0,0-8,0V28H104a4,4,0,0,0,0,8h20V84.26a49.31,49.31,0,0,0-8.57-10C105.15,65.06,91.15,60,76,60a56.06,56.06,0,0,0-56,56c0,28.36,13.79,46.38,25.37,56.51A85.57,85.57,0,0,0,68,186.74V208a12,12,0,0,0,12,12h96a12,12,0,0,0,12-12V186.74a85.57,85.57,0,0,0,22.63-14.23C222.21,162.38,236,144.36,236,116A56.06,56.06,0,0,0,180,60Zm25.59,106.29a75.53,75.53,0,0,1-22.85,13.92A4,4,0,0,0,180,184v24a4,4,0,0,1-4,4H80a4,4,0,0,1-4-4V184a4,4,0,0,0-2.73-3.79,75.38,75.38,0,0,1-22.86-13.92C35.54,153.17,28,136.25,28,116A48.05,48.05,0,0,1,76,68c27.81,0,48,18.5,48,44v64a4,4,0,0,0,8,0V112c0-25.5,20.19-44,48-44a48.05,48.05,0,0,1,48,48C228,136.25,220.46,153.17,205.59,166.29Z"></path>
}.into_view(),
IconWeight::Bold => view! {
    <path d="M180,56c-15.4,0-29.19,4.61-40,12.5V44h12a12,12,0,0,0,0-24H140V12a12,12,0,0,0-24,0v8H104a12,12,0,0,0,0,24h12V68.5C105.19,60.61,91.4,56,76,56a64.07,64.07,0,0,0-64,64c0,31.66,15.53,50.6,28.55,60.91A85.75,85.75,0,0,0,60,192.45V208a20,20,0,0,0,20,20h96a20,20,0,0,0,20-20V192.45a85.75,85.75,0,0,0,19.45-11.54C228.47,170.6,244,151.66,244,120A64.07,64.07,0,0,0,180,56Zm1.09,116.36A12,12,0,0,0,172,184v20H84V184a12,12,0,0,0-9.09-11.64,59.4,59.4,0,0,1-19.46-10.27C42.54,151.87,36,137.71,36,120A40,40,0,0,1,76,80c23.18,0,40,15.14,40,36v60a12,12,0,0,0,24,0V116c0-20.86,16.82-36,40-36a40,40,0,0,1,40,40C220,161.68,182.62,172,181.09,172.36Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M180,58c-19.15,0-35.57,7.79-46,20.32V38h18a6,6,0,0,0,0-12H134V8a6,6,0,0,0-12,0V26H104a6,6,0,0,0,0,12h18V78.32C111.57,65.79,95.15,58,76,58a58.07,58.07,0,0,0-58,58c0,29.11,14.17,47.62,26.05,58a87.74,87.74,0,0,0,22,14V208a14,14,0,0,0,14,14h96a14,14,0,0,0,14-14V188.05A87.74,87.74,0,0,0,212,174c11.88-10.39,26.05-28.9,26.05-58A58.07,58.07,0,0,0,180,58Zm2.1,120.31A6,6,0,0,0,178,184v24a2,2,0,0,1-2,2H80a2,2,0,0,1-2-2V184a6,6,0,0,0-4.1-5.69C73.46,178.16,30,163.13,30,116A46.06,46.06,0,0,1,76,70c26.65,0,46,17.66,46,42v64a6,6,0,0,0,12,0V112c0-24.34,19.35-42,46-42a46.06,46.06,0,0,1,46,46C226,162.9,183.88,177.71,182.1,178.31Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M180,56c-17.74,0-33.21,6.48-44,17.16V40h16a8,8,0,0,0,0-16H136V8a8,8,0,0,0-16,0V24H104a8,8,0,0,0,0,16h16V73.16C109.21,62.48,93.74,56,76,56a60.07,60.07,0,0,0-60,60c0,29.86,14.54,48.85,26.73,59.52A90.48,90.48,0,0,0,64,189.34V208a16,16,0,0,0,16,16h96a16,16,0,0,0,16-16V189.34a90.48,90.48,0,0,0,21.27-13.82C225.46,164.85,240,145.86,240,116A60.07,60.07,0,0,0,180,56Zm1.47,120.41A8,8,0,0,0,176,184v24H80V184a8,8,0,0,0-5.47-7.59C74.1,176.27,32,161.7,32,116A44.05,44.05,0,0,1,76,72c25.5,0,44,16.82,44,40v64a8,8,0,0,0,16,0V112c0-23.18,18.5-40,44-40a44.05,44.05,0,0,1,44,44C224,161.4,183.18,175.83,181.47,176.41Z"></path>
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
