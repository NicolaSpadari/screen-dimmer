#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

#[derive(Clone, serde::Serialize)]
struct Payload {
    message: String,
}

use tauri::{Manager, CustomMenuItem, SystemTray, SystemTrayMenu, SystemTrayMenuItem, SystemTrayEvent};
use std::{thread, time::Duration};

const SIZE: tauri::Size = tauri::Size::Physical(tauri::PhysicalSize{width: 3440, height: 1440});

fn main() {
    let opt_low = CustomMenuItem::new("opacity_low", "Opacity low");
    let opt_med = CustomMenuItem::new("opacity_medium", "Opacity medium");
    let opt_high = CustomMenuItem::new("opacity_high", "Opacity high");
    let opt_quit = CustomMenuItem::new("quit", "Quit");

    let tray_menu = SystemTrayMenu::new()
        .add_item(opt_low)
        .add_item(opt_med)
        .add_item(opt_high)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(opt_quit);
    let system_tray = SystemTray::new()
        .with_menu(tray_menu);

    tauri::Builder::default()
        .setup(move | app | {
            let window = app.get_window("main").unwrap();
            window.set_min_size(Some(SIZE)).unwrap();
            window.set_size(SIZE).unwrap();
            window.center().unwrap();
            let hwnd = window.hwnd().unwrap().0;
            let _pre_val;
            let hwnd = windows::Win32::Foundation::HWND(hwnd);

            unsafe {
                use windows::Win32::UI::WindowsAndMessaging::*;
                let nindex = GWL_EXSTYLE;
                let style = WS_EX_APPWINDOW | WS_EX_COMPOSITED | WS_EX_LAYERED | WS_EX_TRANSPARENT | WS_EX_TOPMOST;
                _pre_val = SetWindowLongA(hwnd, nindex, style.0 as i32);
            };
            Ok(())
        })
        .system_tray(system_tray)
        .on_system_tray_event(| app, event | match event {
            SystemTrayEvent::MenuItemClick { id, .. } => {
                let item_handle = app.tray_handle().get_item(&id);
                match id.as_str() {
                    "opacity_low" => {
                        let opacity_medium_handle = app.tray_handle().get_item("opacity_medium");
                        let opacity_high_handle = app.tray_handle().get_item("opacity_high");

                        let window = app.get_window("main").unwrap();
                        window.emit("opacity", Payload { message: "20".into() }).unwrap();
                        item_handle.set_selected(true).unwrap();
                        opacity_medium_handle.set_selected(false).unwrap();
                        opacity_high_handle.set_selected(false).unwrap();
                    }
                    "opacity_medium" => {
                        let opacity_low_handle = app.tray_handle().get_item("opacity_low");
                        let opacity_high_handle = app.tray_handle().get_item("opacity_high");
                        
                        let window = app.get_window("main").unwrap();
                        window.emit("opacity", Payload { message: "50".into() }).unwrap();
                        item_handle.set_selected(true).unwrap();
                        opacity_low_handle.set_selected(false).unwrap();
                        opacity_high_handle.set_selected(false).unwrap();
                    }
                    "opacity_high" => {
                        let opacity_low_handle = app.tray_handle().get_item("opacity_low");
                        let opacity_medium_handle = app.tray_handle().get_item("opacity_medium");

                        let window = app.get_window("main").unwrap();
                        window.emit("opacity", Payload { message: "70".into() }).unwrap();
                        item_handle.set_selected(true).unwrap();
                        opacity_low_handle.set_selected(false).unwrap();
                        opacity_medium_handle.set_selected(false).unwrap();
                    }
                    "quit" => {
                        let window = app.get_window("main").unwrap();
                        window.emit("opacity", Payload { message: "0".into() }).unwrap();
                        thread::sleep(Duration::from_secs(1));
                        std::process::exit(0)
                    }
                    _ => {}
                }
            }
            _ => {}
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
