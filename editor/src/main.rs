#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
use events::WindowUpdated;
use std::{
    io::{Read, Write},
    sync::{Arc, RwLock},
};
use tungstenite::WebSocket;
use url::Url;

#[derive(Clone, serde::Serialize)]
struct Payload {
    message: String,
}

fn send_event<T>(socket: &mut WebSocket<T>, event: WindowUpdated)
where
    T: Read + Write,
{
    socket
        .write_message(tungstenite::Message::Binary(
            bincode::serialize(&event).unwrap(),
        ))
        .unwrap();
}

fn main() {
    // Use _response to handle initial window positioning
    let (socket, _response) =
        tungstenite::connect(Url::parse("ws://localhost:9002/socket").unwrap())
            .expect("Can' connect");
    let socket = Arc::new(RwLock::new(socket));
    let window_event_socket = Arc::clone(&socket);
    let setup_socket = Arc::clone(&socket);

    tauri::Builder::default()
        .on_window_event(move |event| {
            let mut window_event_socket = window_event_socket.write().unwrap();
            match event.event() {
                // This should be replaced with position of an existing container (div or something)
                tauri::WindowEvent::Moved(moved) => send_event(
                    &mut window_event_socket,
                    WindowUpdated::Moved {
                        x: moved.x,
                        y: moved.y,
                    },
                ),
                // This should be replaced with size of an existing container (div or something)
                tauri::WindowEvent::Resized(resized) => send_event(
                    &mut window_event_socket,
                    WindowUpdated::Resized {
                        width: resized.width,
                        height: resized.height,
                    },
                ),
                tauri::WindowEvent::Destroyed => {
                    window_event_socket
                        .write_message(tungstenite::Message::Binary(
                            bincode::serialize(&WindowUpdated::Closed).unwrap(),
                        ))
                        .unwrap();
                    window_event_socket.close(None).unwrap()
                }
                _ => (),
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
