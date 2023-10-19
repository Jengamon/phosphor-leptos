/// GENERATED FILE

use leptos::*;
use crate::IconWeight;

#[component]
pub fn HourglassSimpleHigh(
    #[prop(into, default = MaybeSignal::Static(IconWeight::Regular))] weight: MaybeSignal<IconWeight>,
    #[prop(into, default = MaybeSignal::Static("1em".to_string()))] size: MaybeSignal<String>,
    #[prop(into, default = MaybeSignal::Static("currentColor".to_string()))] color: MaybeSignal<String>,
    #[prop(into, default = MaybeSignal::Static(false))] mirrored: MaybeSignal<bool>
) -> impl IntoView {
    let body = move || {
        match weight.get() {
            IconWeight::Fill => view!{ <path d="M211.18,196.56,139.57,128l71.61-68.56a1.59,1.59,0,0,1,.13-.13A16,16,0,0,0,200,32H56A16,16,0,0,0,44.69,59.31a1.59,1.59,0,0,1,.13.13L116.43,128,44.82,196.56a1.59,1.59,0,0,1-.13.13A16,16,0,0,0,56,224H200a16,16,0,0,0,11.32-27.31A1.59,1.59,0,0,1,211.18,196.56ZM56,48h0v0Zm144,0-16.7,16H72.72L56,48ZM56,208l72-68.92L200,208Z"/> }.into_view(),
IconWeight::Duotone => view!{ <path d="M186.48,72,128,128,69.52,72Z" opacity="0.2"/><path d="M211.18,196.56,139.57,128l71.61-68.56a1.59,1.59,0,0,1,.13-.13A16,16,0,0,0,200,32H56A16,16,0,0,0,44.69,59.31a1.59,1.59,0,0,1,.13.13L116.43,128,44.82,196.56a1.59,1.59,0,0,1-.13.13A16,16,0,0,0,56,224H200a16,16,0,0,0,11.32-27.31A1.59,1.59,0,0,1,211.18,196.56ZM56,48h0v0ZM89.43,80h77.14L128,116.92ZM200,48l-16.7,16H72.72L56,48ZM56,208l72-68.92L200,208Z"/> }.into_view(),
IconWeight::Thin => view!{ <path d="M133.78,128l74.68-71.51A12,12,0,0,0,200,36H56a12,12,0,0,0-8.49,20.49l.07.06L122.22,128,47.61,199.45l-.07.06A12,12,0,0,0,56,220H200a12,12,0,0,0,8.42-20.55ZM79.49,76h97L128,122.46ZM52.33,46.47A3.93,3.93,0,0,1,56,44H200a4,4,0,0,1,2.89,6.77L184.87,68H71.13l-18-17.2A3.92,3.92,0,0,1,52.33,46.47ZM203.67,209.53A3.93,3.93,0,0,1,200,212H56a4,4,0,0,1-2.86-6.8L128,133.54l74.8,71.63A3.93,3.93,0,0,1,203.67,209.53Z"/> }.into_view(),
IconWeight::Bold => view!{ <path d="M214,193.68,145.35,128,214,62.32l.18-.18A20,20,0,0,0,200,28H56A20,20,0,0,0,41.87,62.14l.18.18L110.65,128l-68.6,65.68-.18.18A20,20,0,0,0,56,228H200a20,20,0,0,0,14.14-34.14ZM107.75,92h40.5L128,111.39ZM190,52,173.31,68H82.69L66,52ZM66,204l62-59.39L190,204Z"/> }.into_view(),
IconWeight::Light => view!{ <path d="M209.8,198l-73.12-70L209.8,58l.09-.09A14,14,0,0,0,200,34H56a14,14,0,0,0-9.9,23.9l.09.09,73.12,70L46.2,198l-.09.09A14,14,0,0,0,56,222H200a14,14,0,0,0,9.9-23.9ZM84.45,78h87.1L128,119.69ZM54.16,47.23A1.91,1.91,0,0,1,56,46H200a2,2,0,0,1,1.45,3.38L184.08,66H71.92L54.56,49.38A1.91,1.91,0,0,1,54.16,47.23ZM201.84,208.77A1.91,1.91,0,0,1,200,210H56a2,2,0,0,1-1.45-3.38L128,136.31l73.44,70.31A1.91,1.91,0,0,1,201.84,208.77Z"/> }.into_view(),
IconWeight::Regular => view!{ <path d="M211.18,196.56,139.57,128l71.61-68.56a1.59,1.59,0,0,1,.13-.13A16,16,0,0,0,200,32H56A16,16,0,0,0,44.7,59.31l.12.13L116.43,128,44.82,196.56l-.12.13A16,16,0,0,0,56,224H200a16,16,0,0,0,11.32-27.31A1.59,1.59,0,0,1,211.18,196.56ZM56,48h0v0ZM89.43,80h77.14L128,116.92ZM200,48l-16.7,16H72.72L56,48ZM56,208l72-68.92L200,208Z"/> }.into_view()
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