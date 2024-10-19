#![allow(non_snake_case)]
#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use dioxus_logger::tracing::{info, Level};
use freya::prelude::*;
use freya_core::plugins::{FreyaPlugin, PluginEvent};
use winit::window::WindowLevel;

mod monitor;

fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    info!("starting app");

    launch_cfg(
        App,
        LaunchConfig::<()>::builder()
            .with_width(300.0)
            .with_height(200.0)
            .with_window_builder(|builder| {
                builder
                    .with_title("Smart Hosts")
                    .with_resizable(false)
                    .with_window_level(WindowLevel::AlwaysOnTop)
                    .with_decorations(false)
                    .with_transparent(true)
                    .with_visible(false)
            })
            .with_plugin(TrayPlugin::default())
            .build(),
    );
}

#[derive(Default)]
pub struct TrayPlugin {
    tray: Option<tray_icon::TrayIcon>,
}

impl FreyaPlugin for TrayPlugin {
    fn on_event(&mut self, event: &PluginEvent) {
        match event {
            PluginEvent::WindowCreated(_) if self.tray.is_none() => {
                info!("Creating tray icon");

                // We create the icon once the event loop is actually running
                // to prevent issues like https://github.com/tauri-apps/tray-icon/issues/90
                self.tray = Some(create_tray());
            }
            _ => {
                // do nothing
            }
        }
    }
}

fn create_tray() -> tray_icon::TrayIcon {
    use tray_icon::{menu::Menu, Icon, TrayIconBuilder};

    let tray_menu = Menu::new();
    let tray_icon = TrayIconBuilder::new()
        .with_menu(Box::new(tray_menu))
        .with_title("Smart Hosts")
        .with_icon(
            Icon::from_rgba(std::iter::repeat(200).take(4 * 32 * 32).collect(), 32, 32).unwrap(),
        )
        .build()
        .unwrap();

    // We have to request a redraw here to have the icon actually show up.
    // Winit only exposes a redraw method on the Window so we use core-foundation directly.
    #[cfg(target_os = "macos")]
    unsafe {
        use core_foundation::runloop::{CFRunLoopGetMain, CFRunLoopWakeUp};

        let rl = CFRunLoopGetMain();
        CFRunLoopWakeUp(rl);
    }
    tray_icon
}

#[component]
fn App() -> Element {
    let mut count = use_signal(|| 0);

    let mut m = crate::monitor::Monitor::new();
    let ssid = m.start();

    rsx!(
        rect {
            height: "50%",
            width: "100%",
            main_align: "center",
            cross_align: "center",
            background: "rgb(0, 119, 182)",
            color: "white",
            shadow: "0 4 20 5 rgb(0, 0, 0, 80)",
            label {
                font_size: "75",
                font_weight: "bold",
                "{count} {ssid}"
            }
        }
        rect {
            height: "50%",
            width: "100%",
            main_align: "center",
            cross_align: "center",
            direction: "horizontal",
            Button {
                onclick: move |_| count += 1,
                label { "Increase" }
            }
            Button {
                onclick: move |_| count -= 1,
                label { "Decrease" }
            }
        }
    )
}
