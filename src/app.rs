use leptos::*;

use super::preferences::Preferences;
use super::tray::Tray;
use super::utils::{get_current_window, log::debug};

#[component]
pub fn App() -> impl IntoView {
    let win = get_current_window();
    let label = win.get_label();

    spawn_local(debug!("Current Window Label: {}", label));

    view! {
        {move || match label.as_str() {
            "Tray" => {
                view! { <Tray /> }
            }
            "Preferences" => {
                view! { <Preferences /> }
            }
            _ => unreachable!(),
        }}
    }
}
