use bevy::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{
    RtcPeerConnection, RtcDataChannel, RtcDataChannelEvent,
    RtcPeerConnectionIceEvent, RtcIceCandidate, RtcSessionDescription,
    RtcSessionDescriptionInit, MessageEvent, WebSocket, BinaryType,
};
use js_sys::{Object, Reflect, JSON};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

// Wrapper types to make web-sys types thread-safe for Bevy
#[derive(Clone)]
struct WebSocketWrapper(Arc<Mutex<Option<WebSocket>>>);

#[derive(Clone)]
struct RtcPeerConnectionWrapper(Arc<Mutex<Option<RtcPeerConnection>>>);

#[derive(Clone)]
struct RtcDataChannelWrapper(Arc<Mutex<Option<RtcDataChannel>>>);

// Implement Send and Sync for our wrappers (safe in WASM single-threaded environment)
unsafe impl Send for WebSocketWrapper {}
unsafe impl Sync for WebSocketWrapper {}
unsafe impl Send for RtcPeerConnectionWrapper {}
unsafe impl Sync for RtcPeerConnectionWrapper {}
unsafe impl Send for RtcDataChannelWrapper {}
unsafe impl Sync for RtcDataChannelWrapper {}

pub struct NetworkPlugin;

impl Plugin for NetworkPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<NetworkState>()
            .init_resource::<PeerConnections>()
            .add_systems(Startup, setup_networking)
            .add_systems(Update, (
                handle_signaling_messages,
                handle_peer_messages,
                broadcast_game_state,
            ));
    }
}

#[derive(Resource)]
pub struct NetworkState {
    pub player_id: String,
    pub room_id: Option<String>,
    pub signaling_socket: WebSocketWrapper,
    pub is_connected: bool,
}

impl Default for NetworkState {
    fn default() -> Self {
        Self {
            player_id: generate_player_id(),
            room_id: None,
            signaling_socket: WebSocketWrapper(Arc::new(Mutex::new(None))),
            is_connected: false,
        }
    }
}

#[derive(Resource)]
pub struct PeerConnections {
    pub connections: HashMap<String, RtcPeerConnectionWrapper>,
    pub data_channels: HashMap<String, RtcDataChannelWrapper>,
}

impl Default for PeerConnections {
    fn default() -> Self {
        Self {
            connections: HashMap::new(),
            data_channels: HashMap::new(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum SignalingMessage {
    Join { room_id: String, player_id: String },
    Leave { player_id: String },
    Offer { from: String, to: String, sdp: String },
    Answer { from: String, to: String, sdp: String },
    IceCandidate { from: String, to: String, candidate: String },
    RoomUpdate { players: Vec<String> },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum GameMessage {
    PlayerUpdate {
        player_id: String,
        position: [f32; 2],
        velocity: [f32; 2],
    },
    GameStateSync {
        state: crate::game::GameState,
    },
    PlayerJoined {
        player_id: String,
    },
    PlayerLeft {
        player_id: String,
    },
}

fn generate_player_id() -> String {
    format!("player_{}", js_sys::Math::random().to_string().chars().skip(2).take(8).collect::<String>())
}

fn setup_networking(
    mut network_state: ResMut<NetworkState>,
) {
    #[cfg(target_arch = "wasm32")]
    {
        // Connect to signaling server
        let ws_url = get_signaling_url();
        if let Ok(ws) = WebSocket::new(&ws_url) {
            ws.set_binary_type(BinaryType::Arraybuffer);
            
            // Set up WebSocket event handlers
            let onopen = Closure::wrap(Box::new(move || {
                web_sys::console::log_1(&"Connected to signaling server".into());
            }) as Box<dyn FnMut()>);
            
            ws.set_onopen(Some(onopen.as_ref().unchecked_ref()));
            onopen.forget();
            
            network_state.signaling_socket = Some(ws);
            network_state.is_connected = true;
        }
    }
}

fn get_signaling_url() -> String {
    // Check if we're running locally or on GitHub Pages
    let window = web_sys::window().unwrap();
    let location = window.location();
    let hostname = location.hostname().unwrap();
    
    if hostname == "localhost" || hostname == "127.0.0.1" {
        "ws://localhost:8080".to_string()
    } else {
        // For GitHub Pages, use a public signaling server or your own deployed server
        "wss://your-signaling-server.herokuapp.com".to_string()
    }
}

fn handle_signaling_messages(
    network_state: Res<NetworkState>,
    mut peer_connections: ResMut<PeerConnections>,
) {
    #[cfg(target_arch = "wasm32")]
    {
        if let Some(ws) = &network_state.signaling_socket {
            // Handle incoming WebSocket messages
            // This would typically be done with event handlers
        }
    }
}

fn handle_peer_messages(
    peer_connections: Res<PeerConnections>,
    mut game_state: ResMut<crate::game::GameState>,
) {
    // Handle incoming messages from peer data channels
    for (peer_id, channel) in peer_connections.data_channels.iter() {
        // Process messages from each peer
    }
}

fn broadcast_game_state(
    peer_connections: Res<PeerConnections>,
    game_state: Res<crate::game::GameState>,
    player_query: Query<&crate::game::Player, With<crate::game::LocalPlayer>>,
) {
    if game_state.is_changed() {
        // Broadcast game state to all connected peers
        let message = GameMessage::GameStateSync {
            state: game_state.clone(),
        };
        
        if let Ok(json_str) = serde_json::to_string(&message) {
            for (_, channel) in peer_connections.data_channels.iter() {
                if channel.ready_state() == web_sys::RtcDataChannelState::Open {
                    let _ = channel.send_with_str(&json_str);
                }
            }
        }
    }
    
    // Broadcast local player updates
    for player in player_query.iter() {
        let message = GameMessage::PlayerUpdate {
            player_id: player.id.clone(),
            position: [player.position.x, player.position.y],
            velocity: [player.velocity.x, player.velocity.y],
        };
        
        if let Ok(json_str) = serde_json::to_string(&message) {
            for (_, channel) in peer_connections.data_channels.iter() {
                if channel.ready_state() == web_sys::RtcDataChannelState::Open {
                    let _ = channel.send_with_str(&json_str);
                }
            }
        }
    }
}

#[cfg(target_arch = "wasm32")]
pub async fn create_peer_connection(peer_id: String) -> Result<RtcPeerConnection, JsValue> {
    let window = web_sys::window().unwrap();
    let rtc_peer_connection = Reflect::get(&window, &JsValue::from_str("RTCPeerConnection"))?;
    let rtc_peer_connection = rtc_peer_connection.dyn_into::<js_sys::Function>()?;
    
    let ice_servers = js_sys::Array::new();
    let stun_server = Object::new();
    Reflect::set(&stun_server, &JsValue::from_str("urls"), &JsValue::from_str("stun:stun.l.google.com:19302"))?;
    ice_servers.push(&stun_server);
    
    let config = Object::new();
    Reflect::set(&config, &JsValue::from_str("iceServers"), &ice_servers)?;
    
    let peer_connection = rtc_peer_connection.construct_with_args(&js_sys::Array::of1(&config))?;
    let peer_connection: RtcPeerConnection = peer_connection.dyn_into()?;
    
    Ok(peer_connection)
}

#[cfg(target_arch = "wasm32")]
pub fn create_data_channel(peer_connection: &RtcPeerConnection, label: &str) -> RtcDataChannel {
    let data_channel_init = web_sys::RtcDataChannelInit::new();
    data_channel_init.set_ordered(true);
    
    peer_connection.create_data_channel_with_data_channel_dict(label, &data_channel_init)
}