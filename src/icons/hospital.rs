//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[cfg(any(feature = "map", feature = "health"))]
#[component]
pub fn Hospital(
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
                <path d="M248,208h-8V128a16,16,0,0,0-16-16H168V48a16,16,0,0,0-16-16H56A16,16,0,0,0,40,48V208H32a8,8,0,0,0,0,16H248a8,8,0,0,0,0-16Zm-120,0H80V160h48Zm0-104H112v16a8,8,0,0,1-16,0V104H80a8,8,0,0,1,0-16H96V72a8,8,0,0,1,16,0V88h16a8,8,0,0,1,0,16Zm96,104H168V128h56Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path
        d="M160,48V216H128V160H80v56H48V48a8,8,0,0,1,8-8h96A8,8,0,0,1,160,48Z"
        opacity="0.2"
    ></path>
    <path d="M248,208h-8V128a16,16,0,0,0-16-16H168V48a16,16,0,0,0-16-16H56A16,16,0,0,0,40,48V208H32a8,8,0,0,0,0,16H248a8,8,0,0,0,0-16Zm-24-80v80H168V128ZM56,48h96V208H136V160a8,8,0,0,0-8-8H80a8,8,0,0,0-8,8v48H56Zm64,160H88V168h32ZM72,96a8,8,0,0,1,8-8H96V72a8,8,0,0,1,16,0V88h16a8,8,0,0,1,0,16H112v16a8,8,0,0,1-16,0V104H80A8,8,0,0,1,72,96Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M248,212H236V128a12,12,0,0,0-12-12H164V48a12,12,0,0,0-12-12H56A12,12,0,0,0,44,48V212H32a4,4,0,0,0,0,8H248a4,4,0,0,0,0-8Zm-24-88a4,4,0,0,1,4,4v84H164V124ZM52,48a4,4,0,0,1,4-4h96a4,4,0,0,1,4,4V212H132V160a4,4,0,0,0-4-4H80a4,4,0,0,0-4,4v52H52Zm72,164H84V164h40ZM76,96a4,4,0,0,1,4-4h20V72a4,4,0,0,1,8,0V92h20a4,4,0,0,1,0,8H108v20a4,4,0,0,1-8,0V100H80A4,4,0,0,1,76,96Z"></path>
}.into_view(),
IconWeight::Bold => view! {
    <path d="M244,204h-4V128a20,20,0,0,0-20-20H172V48a20,20,0,0,0-20-20H56A20,20,0,0,0,36,48V204H32a12,12,0,0,0,0,24H244a12,12,0,0,0,0-24Zm-28-72v72H172V132ZM60,52h88V204H136V160a12,12,0,0,0-12-12H84a12,12,0,0,0-12,12v44H60Zm52,152H96V172h16ZM72,96A12,12,0,0,1,84,84h8V76a12,12,0,0,1,24,0v8h8a12,12,0,0,1,0,24h-8v8a12,12,0,0,1-24,0v-8H84A12,12,0,0,1,72,96Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M248,210H238V128a14,14,0,0,0-14-14H166V48a14,14,0,0,0-14-14H56A14,14,0,0,0,42,48V210H32a6,6,0,0,0,0,12H248a6,6,0,0,0,0-12Zm-24-84a2,2,0,0,1,2,2v82H166V126ZM54,48a2,2,0,0,1,2-2h96a2,2,0,0,1,2,2V210H134V160a6,6,0,0,0-6-6H80a6,6,0,0,0-6,6v50H54Zm68,162H86V166h36ZM74,96a6,6,0,0,1,6-6H98V72a6,6,0,0,1,12,0V90h18a6,6,0,0,1,0,12H110v18a6,6,0,0,1-12,0V102H80A6,6,0,0,1,74,96Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M248,208h-8V128a16,16,0,0,0-16-16H168V48a16,16,0,0,0-16-16H56A16,16,0,0,0,40,48V208H32a8,8,0,0,0,0,16H248a8,8,0,0,0,0-16Zm-24-80v80H168V128ZM56,48h96V208H136V160a8,8,0,0,0-8-8H80a8,8,0,0,0-8,8v48H56Zm64,160H88V168h32ZM72,96a8,8,0,0,1,8-8H96V72a8,8,0,0,1,16,0V88h16a8,8,0,0,1,0,16H112v16a8,8,0,0,1-16,0V104H80A8,8,0,0,1,72,96Z"></path>
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
