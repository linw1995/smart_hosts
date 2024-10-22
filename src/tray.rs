use gloo_utils::format::JsValueSerdeExt;
use leptos::*;

use super::utils::invoke_without_args;
use smart_hosts_bridge::NetworkEvent;

#[component]
pub fn Tray() -> impl IntoView {
    let open_preferences = move |_| {
        spawn_local(async move {
            invoke_without_args("open_preferences").await;
        });
    };

    view! {
        <main class="p-4 flex flex-col justify-between w-screen h-screen">
            <button class="btn glass" on:click=open_preferences>
                "Open Preferences"
            </button>

            <NetworkStat />
        </main>
    }
}

#[component]
pub fn NetworkStat() -> impl IntoView {
    let once = create_resource(
        || (),
        |_| async move {
            let msg = invoke_without_args("monitor").await;
            let msg: NetworkEvent = msg.into_serde().unwrap();
            msg
        },
    );
    {
        move || match once.get() {
            None => view! { <p>"Loading..."</p> }.into_view(),
            Some(data) => {
                use NetworkEvent::*;

                let msg = match data {
                    WiFi { ssid, interface } => format!("WiFI: {} ({})", ssid, interface),
                    Cellular { interface } => format!("Cellular ({})", interface),
                    Wired { interface } => format!("Wired ({})", interface),
                    Unknown => "Unknown".to_string(),
                };

                view! {
                    <div class="stats shadow">
                        <div class="stat">
                            <div class="stat-title">"Current Network"</div>
                            <div class="stat-value">{msg}</div>
                            <div class="stat-desc"></div>
                        </div>
                    </div>
                }
                .into_view()
            }
        }
    }
}
