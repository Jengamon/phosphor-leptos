/// GENERATED FILE

use leptos::*;
use crate::IconWeight;

#[component]
pub fn Balloon(
    #[prop(into, default = MaybeSignal::Static(IconWeight::Regular))] weight: MaybeSignal<IconWeight>,
    #[prop(into, default = MaybeSignal::Static("1em".to_string()))] size: MaybeSignal<String>,
    #[prop(into, default = MaybeSignal::Static("currentColor".to_string()))] color: MaybeSignal<String>,
    #[prop(into, default = MaybeSignal::Static(false))] mirrored: MaybeSignal<bool>
) -> impl IntoView {
    let body = move || {
        match weight.get() {
            IconWeight::Fill => view!{ <path d="M128,16a88.1,88.1,0,0,0-88,88c0,23.43,9.4,49.42,25.13,69.5,12.08,15.41,26.5,26,41.91,31.09L96.65,228.85A8,8,0,0,0,104,240h48a8,8,0,0,0,7.35-11.15L149,204.59c15.4-5.07,29.83-15.68,41.91-31.09C206.6,153.42,216,127.43,216,104A88.1,88.1,0,0,0,128,16Zm49.32,87.89A8.52,8.52,0,0,1,176,104a8,8,0,0,1-7.88-6.68,41.29,41.29,0,0,0-33.43-33.43,8,8,0,1,1,2.64-15.78,57.5,57.5,0,0,1,46.57,46.57A8,8,0,0,1,177.32,103.89Z"/> }.into_view(),
IconWeight::Duotone => view!{ <path d="M137.89,199.13h0L152,232H104l14.09-32.87h0C78.59,192.18,48,144.83,48,104a80,80,0,0,1,160,0C208,144.83,177.41,192.18,137.89,199.13Z" opacity="0.2"/><path d="M128,16a88.1,88.1,0,0,0-88,88c0,23.43,9.4,49.42,25.13,69.5,12.08,15.41,26.5,26,41.91,31.09L96.65,228.85A8,8,0,0,0,104,240h48a8,8,0,0,0,7.35-11.15L149,204.59c15.4-5.07,29.83-15.68,41.91-31.09C206.6,153.42,216,127.43,216,104A88.1,88.1,0,0,0,128,16Zm11.87,208H116.13l6.94-16.19c1.64.12,3.28.19,4.93.19s3.29-.07,4.93-.19Zm38.4-60.37C163.94,181.93,146.09,192,128,192s-35.94-10.07-50.27-28.37C64.12,146.27,56,124,56,104a72,72,0,0,1,144,0C200,124,191.88,146.27,178.27,163.63Zm-1-59.74A8.52,8.52,0,0,1,176,104a8,8,0,0,1-7.88-6.68,41.29,41.29,0,0,0-33.43-33.43,8,8,0,1,1,2.64-15.78,57.5,57.5,0,0,1,46.57,46.57A8,8,0,0,1,177.32,103.89Z"/> }.into_view(),
IconWeight::Thin => view!{ <path d="M128,20a84.09,84.09,0,0,0-84,84c0,22.57,9.08,47.63,24.28,67,12.68,16.17,28,26.81,44.22,31l-12.18,28.4A4,4,0,0,0,104,236h48a4,4,0,0,0,3.68-5.58L143.5,202c16.23-4.17,31.54-14.81,44.22-31,15.2-19.41,24.28-44.47,24.28-67A84.09,84.09,0,0,0,128,20Zm17.93,208H110.07l10.48-24.46a60.56,60.56,0,0,0,14.9,0ZM128,196c-42.1,0-76-50.33-76-92a76,76,0,0,1,152,0C204,145.67,170.1,196,128,196ZM180,95.34a4,4,0,0,1-3.29,4.61,5,5,0,0,1-.66,0,4,4,0,0,1-3.95-3.34A45.31,45.31,0,0,0,135.34,60a4,4,0,0,1,1.32-7.9A53.46,53.46,0,0,1,180,95.34Z"/> }.into_view(),
IconWeight::Bold => view!{ <path d="M128,12a92.1,92.1,0,0,0-92,92c0,24.53,9.55,50.13,26.19,70.22,10,12,21.56,21.07,34.05,26.76L85,227.27A12,12,0,0,0,96,244h64a12,12,0,0,0,11-16.73L159.76,201c12.49-5.69,24.08-14.73,34.05-26.76C210.45,154.13,220,128.53,220,104A92.1,92.1,0,0,0,128,12Zm13.8,208H114.2l5.35-12.49a73.1,73.1,0,0,0,16.9,0Zm33.53-61.09C161.93,175.09,145.12,184,128,184s-33.93-8.91-47.33-25.09C67.73,143.29,60,122.76,60,104a68,68,0,0,1,136,0C196,122.76,188.27,143.29,175.33,158.91Zm-6.34-47q-.6.06-1.2.06a12,12,0,0,1-11.93-10.81,28,28,0,0,0-19.47-23.91,12,12,0,1,1,7.22-22.89,51.94,51.94,0,0,1,36.13,44.42A12,12,0,0,1,169,111.94Z"/> }.into_view(),
IconWeight::Light => view!{ <path d="M128,18a86.1,86.1,0,0,0-86,86c0,23,9.24,48.52,24.71,68.27,12.37,15.79,27.23,26.42,43.05,31.07l-11.27,26.3A6,6,0,0,0,104,238h48a6,6,0,0,0,5.51-8.36l-11.27-26.3c15.82-4.65,30.68-15.28,43-31.07C204.76,152.52,214,127,214,104A86.1,86.1,0,0,0,128,18ZM112.17,204l.58.14a2.05,2.05,0,0,1-.58-.14Zm30.73,22H113.1l8.7-20.31a62.15,62.15,0,0,0,12.4,0ZM128,194c-33.52,0-74-40.15-74-90a74,74,0,0,1,148,0C202,153.85,161.52,194,128,194Zm49-92.08a6.74,6.74,0,0,1-1,.08,6,6,0,0,1-5.91-5A43.29,43.29,0,0,0,135,61.92a6,6,0,1,1,2-11.84A55.48,55.48,0,0,1,181.92,95,6,6,0,0,1,177,101.92Z"/> }.into_view(),
IconWeight::Regular => view!{ <path d="M128,16a88.1,88.1,0,0,0-88,88c0,23.43,9.4,49.42,25.13,69.5,12.08,15.41,26.5,26,41.91,31.09L96.65,228.85A8,8,0,0,0,104,240h48a8,8,0,0,0,7.35-11.15L149,204.59c15.4-5.07,29.83-15.68,41.91-31.09C206.6,153.42,216,127.43,216,104A88.1,88.1,0,0,0,128,16Zm11.87,208H116.13l6.94-16.19c1.64.12,3.28.19,4.93.19s3.29-.07,4.93-.19Zm38.4-60.37C163.94,181.93,146.09,192,128,192s-35.94-10.07-50.27-28.37C64.12,146.27,56,124,56,104a72,72,0,0,1,144,0C200,124,191.88,146.27,178.27,163.63Zm-1-59.74A8.52,8.52,0,0,1,176,104a8,8,0,0,1-7.88-6.68,41.29,41.29,0,0,0-33.43-33.43,8,8,0,1,1,2.64-15.78,57.5,57.5,0,0,1,46.57,46.57A8,8,0,0,1,177.32,103.89Z"/> }.into_view()
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