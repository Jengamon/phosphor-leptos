/// GENERATED FILE

use leptos::*;
use crate::IconWeight;

#[component]
pub fn SpeakerHifi(
    #[prop(into, default = MaybeSignal::Static(IconWeight::Regular))] weight: MaybeSignal<IconWeight>,
    #[prop(into, default = MaybeSignal::Static("1em".to_string()))] size: MaybeSignal<String>,
    #[prop(into, default = MaybeSignal::Static("currentColor".to_string()))] color: MaybeSignal<String>,
    #[prop(into, default = MaybeSignal::Static(false))] mirrored: MaybeSignal<bool>
) -> impl IntoView {
    let body = move || {
        match weight.get() {
            IconWeight::Fill => view!{ <path d="M152,160a24,24,0,1,1-24-24A24,24,0,0,1,152,160ZM208,40V216a16,16,0,0,1-16,16H64a16,16,0,0,1-16-16V40A16,16,0,0,1,64,24H192A16,16,0,0,1,208,40ZM116,68a12,12,0,1,0,12-12A12,12,0,0,0,116,68Zm52,92a40,40,0,1,0-40,40A40,40,0,0,0,168,160Z"/> }.into_view(),
IconWeight::Duotone => view!{ <path d="M192,32H64a8,8,0,0,0-8,8V216a8,8,0,0,0,8,8H192a8,8,0,0,0,8-8V40A8,8,0,0,0,192,32ZM128,184a32,32,0,1,1,32-32A32,32,0,0,1,128,184Z" opacity="0.2"/><path d="M192,24H64A16,16,0,0,0,48,40V216a16,16,0,0,0,16,16H192a16,16,0,0,0,16-16V40A16,16,0,0,0,192,24Zm0,192H64V40H192ZM116,76a12,12,0,1,1,12,12A12,12,0,0,1,116,76Zm12,116a40,40,0,1,0-40-40A40,40,0,0,0,128,192Zm0-64a24,24,0,1,1-24,24A24,24,0,0,1,128,128Z"/> }.into_view(),
IconWeight::Thin => view!{ <path d="M192,28H64A12,12,0,0,0,52,40V216a12,12,0,0,0,12,12H192a12,12,0,0,0,12-12V40A12,12,0,0,0,192,28Zm4,188a4,4,0,0,1-4,4H64a4,4,0,0,1-4-4V40a4,4,0,0,1,4-4H192a4,4,0,0,1,4,4ZM120,76a8,8,0,1,1,8,8A8,8,0,0,1,120,76Zm8,40a36,36,0,1,0,36,36A36,36,0,0,0,128,116Zm0,64a28,28,0,1,1,28-28A28,28,0,0,1,128,180Z"/> }.into_view(),
IconWeight::Bold => view!{ <path d="M192,20H64A20,20,0,0,0,44,40V216a20,20,0,0,0,20,20H192a20,20,0,0,0,20-20V40A20,20,0,0,0,192,20Zm-4,192H68V44H188ZM112,76a16,16,0,1,1,16,16A16,16,0,0,1,112,76Zm16,120a44,44,0,1,0-44-44A44.05,44.05,0,0,0,128,196Zm0-64a20,20,0,1,1-20,20A20,20,0,0,1,128,132Z"/> }.into_view(),
IconWeight::Light => view!{ <path d="M192,26H64A14,14,0,0,0,50,40V216a14,14,0,0,0,14,14H192a14,14,0,0,0,14-14V40A14,14,0,0,0,192,26Zm2,190a2,2,0,0,1-2,2H64a2,2,0,0,1-2-2V40a2,2,0,0,1,2-2H192a2,2,0,0,1,2,2ZM118,76a10,10,0,1,1,10,10A10,10,0,0,1,118,76Zm10,38a38,38,0,1,0,38,38A38,38,0,0,0,128,114Zm0,64a26,26,0,1,1,26-26A26,26,0,0,1,128,178Z"/> }.into_view(),
IconWeight::Regular => view!{ <path d="M192,24H64A16,16,0,0,0,48,40V216a16,16,0,0,0,16,16H192a16,16,0,0,0,16-16V40A16,16,0,0,0,192,24Zm0,192H64V40H192ZM116,76a12,12,0,1,1,12,12A12,12,0,0,1,116,76Zm12,116a40,40,0,1,0-40-40A40,40,0,0,0,128,192Zm0-64a24,24,0,1,1-24,24A24,24,0,0,1,128,128Z"/> }.into_view()
        }
    };

    let transform = move || if mirrored.get() { "scale(-1, 1)" } else { "" };

    view! {
        <svg 
            xmlns="http://www.w3.org/2000/svg" 
            width=size.clone()
            height=size
            fill=color
            transform=transform
            viewBox="0 0 256 256"
        >
            {body}
        </svg>
    }
}