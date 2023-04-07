#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
use events::WindowUpdated;
use std::sync::{Arc, RwLock};
use tauri::Manager;
use url::Url;

#[derive(Clone, serde::Serialize)]
struct Payload {
    message: String,
}
fn main() {
    let (mut socket, response) =
        tungstenite::connect(Url::parse("ws://localhost:9002/socket").unwrap())
            .expect("Can' connect");
    let socket = Arc::new(RwLock::new(socket));
    let csocket = std::sync::Arc::clone(&socket);

    tauri::Builder::default()
        .on_window_event(move |event| {
            let mut csocket = csocket.write().unwrap();
            match event.event() {
                tauri::WindowEvent::Resized(resized) => {
                    csocket
                        .write_message(tungstenite::Message::Binary(
                            bincode::serialize(&WindowUpdated {
                                width: resized.width,
                                height: resized.height,
                            })
                            .unwrap(),
                        ))
                        .unwrap();
                }
                tauri::WindowEvent::Destroyed => csocket.close(None).unwrap(),
                _ => (),
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
