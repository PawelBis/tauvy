use bevy::prelude::*;
use events::WindowUpdated;
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex, RwLock};
use tungstenite::WebSocket;

fn main() {
    let server = TcpListener::bind("127.0.0.1:9002").unwrap();
    let (stream, _) = server.accept().unwrap();
    let websocket = tungstenite::accept(stream).unwrap();

    App::new()
        .insert_resource(Resolution::default())
        .insert_resource(EventStream::new(websocket))
        //.insert_resource(server)
        .add_plugins(DefaultPlugins)
        //.add_systems(Startup, (setup_camera, setup_ui))
        .add_startup_systems((setup_camera, setup_ui))
        //.add_systems(Update, (on_resize_system, toggle_resolution))
        .add_system(handle_window_events)
        .run();
}

#[derive(Default, Resource)]
struct Resolution(Vec2);

#[derive(Component)]
struct ResolutionText;

#[derive(Resource)]
struct EventStream {
    websocket: Arc<Mutex<WebSocket<TcpStream>>>,
    pub events: Arc<RwLock<Vec<WindowUpdated>>>,
}

impl EventStream {
    fn new(websocket: WebSocket<TcpStream>) -> Self {
        let mut res = Self {
            websocket: Arc::new(Mutex::new(websocket)),
            events: Arc::default(),
        };

        res.read_stream();
        res
    }

    fn read_stream(&mut self) {
        let socket = Arc::clone(&self.websocket);
        let events = Arc::clone(&self.events);
        std::thread::spawn(move || loop {
            match socket.lock().unwrap().read_message() {
                Ok(m) => match m {
                    tungstenite::Message::Binary(b) => {
                        let m: WindowUpdated = bincode::deserialize(&b).unwrap();
                        let mut events = events.write().unwrap();
                        events.push(m);
                    }
                    something_else => {
                        println!("read nothing?: {something_else:?}");
                    }
                },
                Err(_) => {
                    break;
                }
            };
        });
    }

    fn get_window_events(&mut self) -> Vec<WindowUpdated> {
        let mut events = self.events.write().unwrap();
        let mut out: Vec<WindowUpdated> = Vec::new();
        std::mem::swap(&mut *events, &mut out);

        return out;
    }
}

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

fn handle_window_events(
    mut event_stream: ResMut<EventStream>,
    mut app_exit_events: ResMut<Events<bevy::app::AppExit>>,
    mut windows: Query<&mut Window>,
) {
    let mut window = windows.single_mut();
    for e in event_stream.get_window_events() {
        match e {
            WindowUpdated::Moved { x, y } => {
                window.position.set(IVec2{ x, y });
            }
            WindowUpdated::Resized { width, height, } => { 
                window.resolution.set(width as f32, height as f32);
            },
            WindowUpdated::Closed => { app_exit_events.send(bevy::app::AppExit); },
        }
    }
}
