use gloo_utils::format::JsValueSerdeExt;
use leptos::*;

use super::utils::invoke_without_args;
use smart_hosts_bridge::NetworkEvent;

#[component]
pub fn Tray() -> impl IntoView {
    let once = create_resource(
        || (),
        |_| async move {
            let msg = invoke_without_args("monitor").await;
            let msg: NetworkEvent = msg.into_serde().unwrap();
            msg
        },
    );
    view! {
        <main class="container">
            {move || match once.get() {
                None => view! { <p>"Loading..."</p> }.into_view(),
                Some(data) => view! { <p>{move || format!("{:?}", data) }</p> }.into_view()
            }}
        </main>
    }
}
