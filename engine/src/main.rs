use bevy::{prelude::*, window::WindowResized};
use events::WindowUpdated;
use std::net::{TcpListener, TcpStream};
use tungstenite::handshake::server::{Request, Response};
use tungstenite::Error;
use tungstenite::WebSocket;

fn main() {
    let server = TcpListener::bind("127.0.0.1:9002").unwrap();
    let (stream, _) = server.accept().unwrap();
    let websocket = tungstenite::accept(stream).unwrap();

    App::new()
        .insert_resource(Resolution::default())
        .insert_resource(EventStream(websocket))
        //.insert_resource(server)
        .add_plugins(DefaultPlugins)
        //.add_systems(Startup, (setup_camera, setup_ui))
        .add_startup_systems((setup_camera, setup_ui))
        //.add_systems(Update, (on_resize_system, toggle_resolution))
        .add_systems((on_resize_system, update_resolution))
        .add_system(read_stream)
        .run();
}

#[derive(Default, Resource)]
struct Resolution(Vec2);

#[derive(Component)]
struct ResolutionText;

#[derive(Resource)]
struct EventStream(WebSocket<TcpStream>);

fn setup_camera(mut cmd: Commands) {
    cmd.spawn(Camera2dBundle::default());
}

fn setup_ui(mut cmd: Commands, _asset_server: Res<AssetServer>) {
    cmd.spawn(NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
            ..default()
        },
        ..default()
    })
    .with_children(|root| {
        root.spawn((
            TextBundle::from_section(
                "Resolution",
                TextStyle {
                    //font: asset_server.load(),
                    font_size: 50.0,
                    color: Color::BLACK,
                    ..default()
                },
            ),
            ResolutionText,
        ));
    });
}

fn update_resolution(mut windows: Query<&mut Window>, resolution: Res<Resolution>) {
    let mut _window = windows.single_mut();
    // window.resolution.set(resolution.0.x, resolution.0.y);
}

fn on_resize_system(
    mut q: Query<&mut Text, With<ResolutionText>>,
    mut resize_reader: EventReader<WindowResized>,
) {
    let mut text = q.single_mut();
    for e in resize_reader.iter() {
        text.sections[0].value = format!("{:.1} x {:.1}", e.width, e.height);
    }
}

fn read_stream(
    mut socket: ResMut<EventStream>,
    mut app_exit_events: ResMut<Events<bevy::app::AppExit>>,
) {
    // Just consume all events for now
    loop {
        match socket.0.read_message() {
            Ok(m) => match m {
                tungstenite::Message::Binary(b) => {
                    let m: WindowUpdated = bincode::deserialize(&b).unwrap();
                    println!("{m:?}")
                }
                _ => {},
            },
            Err(e) => {
                println!("{e:?}");
                if let Error::ConnectionClosed | Error::AlreadyClosed = e {
                    app_exit_events.send(bevy::app::AppExit);
                }
                break;
            }
        };
    }
}
