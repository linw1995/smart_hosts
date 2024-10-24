use leptos::*;

use crate::utils::{load_theme, save_theme, ThemeCtx};

use super::preferences::Preferences;
use super::tray::Tray;
use super::utils::{get_current_window, log::debug};

#[component]
pub fn App() -> impl IntoView {
    let theme = load_theme().unwrap_or("light".to_string());
    let (theme, set_theme) = create_signal(theme);
    provide_context(ThemeCtx(set_theme));
    create_effect(move |_| {
        let theme: String = theme.get();
        save_theme(&theme);
    });

    let win = get_current_window();
    let label = win.get_label();

    spawn_local(debug!("Current Window Label: {}", label));

    view! {
        <main data-theme=theme>
            {move || match label.as_str() {
                "Tray" => {
                    view! { <Tray /> }
                }
                "Preferences" => {
                    view! { <Preferences /> }
                }
                _ => unreachable!(),
            }}
        </main>
    }
}
